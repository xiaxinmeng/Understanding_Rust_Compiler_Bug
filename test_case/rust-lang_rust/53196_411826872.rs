plain
[00:45:02] ....................................................................................................
[00:45:04] ........................i...........................................................................
[00:45:07] ...........................................................................i.i..ii..................
[00:45:11] ....................................................................................................
[00:45:14] ..............................................................F....i................................
[00:45:20] ....................................................................................................
[00:45:22] ....................................................................................................
[00:45:25] ....................................................................................................
[00:45:29] ....i...............................................................................................
---
[00:45:47] 
[00:45:47] ---- [ui] ui/nolink-with-link-args.rs stdout ----
[00:45:47] diff of stderr:
[00:45:47] 
[00:45:47] 1 error: linking with `ld` failed: exit code: 1
[00:45:47] 2    |
[00:45:47] -    = note: "ld" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "$TEST_BUILD_DIR/nolink-with-link-args/a.nolink_with_link_args0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "$TEST_BUILD_DIR/nolink-with-link-args/a.nolink_with_link_args1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "$TEST_BUILD_DIR/nolink-with-link-args/a" "--gc-sections" "-pie" "-zrelro" "-znow" "-O1" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "$TEST_BUILD_DIR/nolink-with-link-args/auxiliary" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--start-group" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-41f43a30bc296e4f" "--end-group" "-Bstatic" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-ce73107aecc35a63.rlib" "-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,$LIB_DIR/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "aFdEfSeVEEE"
[00:45:47] +    = note: "ld" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "$TEST_BUILD_DIR/nolink-with-link-args/a.nolink_with_link_args0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "$TEST_BUILD_DIR/nolink-with-link-args/a.nolink_with_link_args1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "$TEST_BUILD_DIR/nolink-with-link-args/a" "--gc-sections" "-pie" "-zrelro" "-znow" "-O1" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "$TEST_BUILD_DIR/nolink-with-link-args/auxiliary" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--start-group" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-2339b911e3c09de8" "--end-group" "-Bstatic" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2866e3dedc0a56e0.rlib" "-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,$LIB_DIR/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "aFdEfSeVEEE"
[00:45:47] 4    = note: ld: unrecognized option '-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib'
[00:45:47] 5            ld: use the --help option for usage information
[00:45:47] 
[00:45:47] 
[00:45:47] 
[00:45:47] The actuanknown-linux-gnu/test/ui/nolink-with-link-args/a.nolink_with_link_args1-317d481089b8c8fe83113de504472633.rs.rcgu.o\" \"-o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a\" \"--gc-sections\" \"-pie\" \"-zrelro\" \"-znow\" \"-O1\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/auxiliary\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"--start-group\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-lstd-2339b911e3c09de8\" \"--end-group\" \"-Bstatic\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2866e3dedc0a56e0.rlib\" \"-Bdynamic\" \"-ldl\" \"-lrt\" \"-lpthread\" \"-lpthread\" \"-lgcc_s\" \"-lc\" \"-lm\" \"-lrt\" \"-lpthread\" \"-lutil\" \"-lutil\" \"-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,--enable-new-dtags\" \"aFdEfSeVEEE\"","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"ld: unrecognized option '-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib'\nld: use the --help option for usage information\n","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: linking with `ld` failed: exit code: 1\n   |\n   = note: \"ld\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a.nolink_with_link_args0-317d481089b8c8fe83113de504472633.rs.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a.nolink_with_link_args1-317d481089b8c8fe83113de504472633.rs.rcgu.o\" \"-o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a\" \"--gc-sections\" \"-pie\" \"-zrelro\" \"-znow\" \"-O1\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/auxiliary\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"--start-group\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-lstd-2339b911e3c09de8\" \"--end-group\" \"-Bstatic\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2866e3dedc0a56e0.rlib\" \"-Bdynamic\" \"-ldl\" \"-lrt\" \"-lpthread\" \"-lpthread\" \"-lgcc_s\" \"-lc\" \"-lm\" \"-lrt\" \"-lpthread\" \"-lutil\" \"-lutil\" \"-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,--enable-new-dtags\" \"aFdEfSeVEEE\"\n   = note: ld: unrecognized option '-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib'\n           ld: use the --help option for usage information\n           \n\n"}
[00:45:47] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:47] ------------------------------------------
[00:45:47] 
[00:45:47] thread '[ui] ui/nolink-with-link-args.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:45:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:47] 
[00:45:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:45:47] 
[00:45:47] 
[00:45:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:47] 
[00:45:47] 
[00:45:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:47] Build completed unsuccessfully in 0:03:01
[00:45:47] Build completed unsuccessfully in 0:03:01
[00:45:47] make: *** [check] Error 1
[00:45:47] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:099ad714
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:07c64c00
$ sudo tail -n 500 /var/log/syslog
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] kvm-clock: using sched offset of 1386118676 cycles
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Zone ranges:
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   Device   empty
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Movable zone start for each node
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Early memory node ranges
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 glob travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Policy zone: Normal
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] console [ttyS0] enabled
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.345924] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.347610] pid_max: default: 32768 minimum: 301
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.349269] ACPI: Core revision 20150930
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.356034] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.357337] Security Framework initialized
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.358001] Yama: becoming mindful.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.358609] AppArmor: AppArmor disabled by boot time parameter
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.361439] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.371426] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.376031] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.377374] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.378640] Initializing cgroup subsys io
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.379371] Initializing cgroup subsys memory
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.380026] Initializing cgroup subsys devices
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.380862] Initializing cgroup subsys freezer
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.381880] Initializing cgroup subsys net_cls
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.382887] Initializing cgroup subsys perf_event
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.383975] Initializing cgroup subsys net_prio
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.384624] Initializing cgroup subsys hugetlb
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.385580] Initializing cgroup subsys pids
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.386336] CPU: Physical Processor ID: 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.387115] CPU: Processor Core ID: 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.393593] mce: CPU supports 32 MCE banks
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.394777] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.395733] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.398529] Freeing SMP alternatives memory: 32K
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.407442] ftrace: allocating 32185 entries in 126 pages
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.456336] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.458177] smpboot: Max logical packages: 2
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.459395] x2apic enabled
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.461193] Switched APIC routing to physical x2apic.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.465533] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.573042] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.575922] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.580656] x86: Booting SMP configuration:
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.581576] .... node  #0, CPUs:      #1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.582717] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.586640]  #2
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.587087] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.591450]  #3
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.592083] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.595718] x86: Booted up 1 node, 4 CPUs
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.596559] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.598938] devtmpfs: initialized
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.603831] evm: security.selinux
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.604360] evm: security.SMACK64
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.604992] evm: security.SMACK64EXEC
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.605814] evm: security.SMACK64TRANSMUTE
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.606568] evm: security.SMACK64MMAP
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.607115] evm: security.ima
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.607623] evm: security.capability
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.608809] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.610705] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.612004] pinctrl core: initialized pinctrl subsystem
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.613497] RTC time: 16:12:35, date: 08/09/18
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.615259] NET: Registered protocol family 16
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.625088] cpuidle: using governor ladder
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.637085] cpuidle: using governor menu
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.638117] PCCT header not found.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.638834] ACPI: bus type PCI registered
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.639407] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.640698] PCI: Using configuration type 1 for base access
Aug  9 16:12:45 travis-job-8d18058fae969 kernel: [    0.725122] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.727954] PCI host bridge to bus 0000:00
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.729547] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.730909] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.732092] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.733618] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.734863] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.735941] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.736381] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.753491] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.771392] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.773012] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.779296] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.785247] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.802035] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.808867] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.815763] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.834219] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.837470] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.840318] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.843065] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.845917] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.867158] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.868796] vgaarb: loaded
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.869635] SCSI subsystem initialized
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.870727] libata version 3.00 loaded.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.870765] ACPI: bus type USB registered
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.871835] usbcore: registered new interface driver usbfs
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.873058] usbcore: registered new interface driver hub
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.874483] usbcore: registered new device driver usb
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.876129] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.879389] dmi: Firmware registration failed.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.880579] PCI: Using ACPI for IRQ routing
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.881247] PCI: pci_cache_line_size set to 64 bytes
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.881347] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.881349] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.881470] NetLabel: Initializing
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.882829] NetLabel:  domain hash size = 128
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.883945] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.884852] NetLabel:  unlabeled traffic allowed by default
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.885789] amd_nb: Cannot enumerate AMD northbridges
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.887273] clocksource: Switched to clocksource kvm-clock
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.898522] pnp: PnP ACPI init
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.899233] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.899318] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.899377] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.899428] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.899471] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.899512] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.899555] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.899750] pnp: PnP ACPI: found 7 devices
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.907678] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.909452] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.909455] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.909457] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.909459] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.909499] NET: Registered protocol family 2
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.910625] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.912081] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.913477] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.915166] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.916393] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.918545] NET: Registered protocol family 1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.919962] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.921030] PCI: CLS 0 bytes, default 64
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    0.921092] Unpacking initramfs...
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.059562] Freeing initrd memory: 21432K
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.060449] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.061851] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.063696] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.064982] hw unit of domain pp0-core 2^-0 Joules
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.065704] hw unit of domain package 2^-0 Joules
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.066622] hw unit of domain dram 2^-0 Joules
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.067522] Scanning for low memory corruption every 60 seconds
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.069257] audit: initializing netlink subsys (disabled)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.070385] audit: type=2000 audit(1533831157.258:1): initialized
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.071929] Initialise system trusted keyring
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.073120] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.074158] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.076433] zbud: loaded
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.077371] VFS: Disk quotas dquot_6.6.0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.078053] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.079262] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.080643] fuse init (API version 7.23)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.081845] Key type big_key registered
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.082618] Allocating IMA MOK and blacklist keyrings.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.085142] Key type asymmetric registered
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.085962] Asymmetric key parser 'x509' registered
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.086859] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.088253] io scheduler noop registered
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.088919] io scheduler deadline registered (default)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.089856] io scheduler cfq registered
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.090779] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.092189] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.093552] intel_idle: does not run on family 6 model 45
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.093657] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.094892] ACPI: Power Button [PWRF]
Aug  9 16:12:45 travis-job-8d18058f- (irq = 7, base_baud = 115200) is a 16550A
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.213787] Linux agpgart interface v0.103
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.217461] loop: module loaded
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.218362] libphy: Fixed MDIO Bus: probed
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.219562] tun: Universal TUN/TAP device driver, 1.6
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.220945] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.265253] PPP generic driver version 2.4.2
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.266597] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.268549] ehci-pci: EHCI PCI platform driver
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.269753] ehci-platform: EHCI generic platform driver
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.271152] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.273060] ohci-pci: OHCI PCI platform driver
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.273989] ohci-platform: OHCI generic platform driver
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.275411] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.277567] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.279671] i8042: Warning: Keylock active
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.281924] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.283377] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.284908] mousedev: PS/2 mouse device common for all mice
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.287316] rtc_cmos 00:00: RTC can wake from S4
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.289132] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.290800] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.292198] i2c /dev entries driver
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.293202] device-mapper: uevent: version 1.0.3
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.294662] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.296712] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.298899] NET: Registered protocol family 10
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.300337] NET: Registered protocol family 17
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.301457] Key type dns_resolver registered
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.302534] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.303989] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.305698] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.307181] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.308767] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.311341] registered taskstats version 1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.312364] Loading compiled-in X.509 certificates
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.314024] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.316591] zswap: loaded using pool lzo/zbud
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.319725] Key type trusted registered
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.323928] Key type encrypted registered
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.324967] ima: No TPM chip found, activating TPM-bypass!
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.326638] evm: HMAC attrs: 0x1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.328108]   Magic number: 14:552:236
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.329234] memory memory114: hash matches
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.330440] rtc_cmos 00:00: setting system clock to 2018-08-09 16:12:37 UTC (1533831157)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.332592] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.334137] EDD information not available.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.335192] PM: Hibernation image not present or could not be loaded.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.336582] Freeing unused kernel memory: 1496K
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.337340] Write protecting the kernel read-only data: 14336k
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.339604] Freeing unused kernel memory: 1956K
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.340819] Freeing unused kernel memory: 92K
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.357263] systemd-udevd[118]: starting version 204
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.417203] scsi host0: Virtio SCSI HBA
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.423300] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.426071] AVX version of gcm_enc/dec engaged.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.426764] AES CTR mode by8 optimization enabled
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.466164] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.466246] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.466248] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.466437] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.466439] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.466492] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.468369]  sda: sda1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.469316] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    3.483916] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    4.067435] tsc: Refined TSC clocksource calibration: 2599.752 MHz
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    4.068374] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257951aaeca, max_idle_ns: 440795203509 ns
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    4.321426] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    6.407533] floppy0: no floppy controllers found
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.563288] raid6: sse2x1   gen()  9089 MB/s
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.631303] raid6: sse2x1   xor()  6678 MB/s
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.699308] raid6: sse2x2   gen() 11550 MB/s
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.767320] raid6: sse2x2   xor()  7732 MB/s
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.835295] raid6: sse2x4   gen() 12857 MB/s
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.903311] raid6: sse2x4   xor()  8992 MB/s
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.904186] raid6: using algorithm sse2x4 gen() 12857 MB/s
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.905070] raid6: .... xor() 8992 MB/s, rmw enabled
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.905858] raid6: using ssse3x2 recovery algorithm
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.908168] xor: automatically using best checksumming function:
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.947304]    avx       : 22378.000 MB/sec
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    7.961542] Btrfs loaded
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    8.003940] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    8.005377] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    8.078761] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    8.085797] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    8.086977] EXT4-fs (sda1): recovery complete
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    8.091907] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    8.295839] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    8.409245] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    8.463076] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    8.652954] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    9.189002] random: cloud-init: uninitialized urandom read (32 bytes read, 47 bits of entropy available)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    9.329616] systemd-udevd[702]: starting version 204
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    9.441210] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    9.482244] intel_rapl: no valid rapl domains found in package 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    9.528883] ppdev: user-space parallel port driver
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    9.654357] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    9.706120] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    9.766831] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [    9.929896] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   10.218740] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   10.299822] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   10.380255] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   10.422712] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   10.820086] init: failsafe main process (1096) killed by TERM signal
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Running set_multiqueue.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Set channels for eth0 to 4.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   11.515301] random: nonblocking pool is initialized
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  9 16:12:45 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Starting Google Accounts daemon.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 cron[1390]: (CRON) INFO (pidfile fd = 3)
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 cron[1426]: (CRON) STARTUP (fork ok)
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 cron[1426]: (CRON) INFO (Running @reboot jobs)
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Creating a new user account for me.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 acpid: starting up with netlink and the input layer
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 acpid: 1 rule loaded
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 acpid: waiting for events: event logging is off
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 haveged: haveged starting up
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Created user account me.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Creating a new user account for henrik.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   12.033644] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Created user account henrik.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   12.050304] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Creating a new user account for emma.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Created user account emma.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Creating a new user account for igor.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Created user account igor.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Created user account konstantinhaase.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Creating a new user account for aj.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Created user account aj.
 google-accounts: INFO Creating a new user account for konstantin.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Created user account konstantin.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Creating a new user account for carmen.
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   12.531465] floppy0: no floppy controllers found
Aug  9 16:12:46 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Created user account carmen.
Aug  9 16:12:47 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 16:12:47 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Creating a new user account for maria.
Aug  9 16:12:47 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Created user account maria.
Aug  9 16:12:47 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 google-accounts: INFO Removing user packer.
Aug  9 16:13:09 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpdate[1849]: adjust time server 169.254.169.254 offset 0.009620 sec
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1884]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: proto: precision = 0.101 usec
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: Listen normally on 3 eth0 10.20.2.126 UDP 123
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: peers refreshed
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: Listening on routing socket on fd #21 for interface updates
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   42.203976] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 startup-script: INFO Found startup-script in metadata.
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 startup-script: INFO startup-script: job 1 at Thu Aug  9 19:23:00 2018
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 startup-script: INFO startup-script: Return code 0.
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 startup-script: INFO startup-script: Return code 0.
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 startup-script: INFO Finished running startup scripts.
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ec2: 
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ec2: #############################################################
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ec2: 1024 9b:15:96:66:cf:57:c2:31:fd:1b:3f:5d:fd:d7:65:d9  root@travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 (DSA)
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ec2: 256 db:66:4b:dd:c1:b7:62:b4:65:31:b9:4b:a5:d5:cb:81  root@travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 (ECDSA)
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ec2: 256 e0:bb:72:a0:a6:7e:23:05:82:0f:ea:94:04:2b:58:37  root@travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 (ED25519)
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ec2: 2048 af:39:1a:73:7d:d3:ee:9c:9d:36:df:44:1f:f2:8e:06  root@travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 (RSA)
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 16:13:16 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ec2: #############################################################
Aug  9 16:13:48 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [   74.326105] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.181112] device veth54d49e7 entered promiscuous mode
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.181183] docker0: port 1(veth54d49e7) entered forwarding state
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.181192] docker0: port 1(veth54d49e7) entered forwarding state
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.181648] docker0: port 1(veth54d49e7) entered disabled state
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.269245] cgroup: docker-runc (4869) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.269249] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.340514] eth0: renamed from veth5ed639e
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.380066] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.381580] docker0: port 1(veth54d49e7) entered forwarding state
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.381597] docker0: port 1(veth54d49e7) entered forwarding state
Aug  9 16:14:54 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  140.381621] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 16:14:57 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: Listen normally on 5 docker0 fe80::42:3eff:fe69:1ea5 UDP 123
Aug  9 16:14:57 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  9 16:14:57 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 16:14:57 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: peers refreshed
Aug  9 16:14:57 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 ntpd[1885]: new interface(s) found: waking up resolver
Aug  9 16:15:09 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [  155.408115] docker0: port 1(veth54d49e7) entered forwarding state
Aug  9 16:17:01 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 CRON[5947]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 16:59:37 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [ 2822.500620] docker0: port 1(veth54d49e7) entered disabled state
Aug  9 16:59:37 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [ 2822.500681] veth5ed639e: renamed from eth0
Aug  9 16:59:37 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [ 2822.558274] docker0: port 1(veth54d49e7) entered disabled state
Aug  9 16:59:37 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [ 2822.560429] device veth54d49e7 left promiscuous mode
Aug  9 16:59:37 travis-job-8d18058f-6ee4-4ac5-a7d6-59f0c7fae969 kernel: [ 2822.560433] docker0: port 1(veth54d49e7) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:2363f408
---
travis_time:end:12c8a07b:start=1533833978603237784,finish=1533833978609515943,duration=6278159
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:187a413e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:066bd3c4
travis_time:start:066bd3c4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0dff9ff8
$ dmesg | grep -i kill
