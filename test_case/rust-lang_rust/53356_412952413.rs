plain
[00:50:49] ....................................................................................................
[00:50:52] ....................................................................................................
[00:50:55] ....................................................................................................
[00:50:59] ....................................................................................................
[00:51:02] .............F........i.............................................................................
[00:51:08] ....................................................................................................
[00:51:11] .................................................................i..................................
[00:51:15] ....................................................................................................
[00:51:18] ....................................................................................................
---
[00:51:46] 
[00:51:46] ---- [ui] ui/lto-duplicate-symbols.rs stdout ----
[00:51:46] diff of stderr:
[00:51:46] 
[00:51:46] 1 warning: Linking globals named 'foo': symbol multiply defined!
[00:51:46] 2 
[00:51:46] - error: failed to load bc of "lto_duplicate_symbols10-8787f43e282added376259c1adb08b80.rs": 
[00:51:46] + error: failed to load bc of "lto_duplicate_symbols1.3a1fbbbh-cgu.0": 
[00:51:46] 5 error: aborting due to previous error
[00:51:46] 6 
[00:51:46] 
[00:51:46] 
[00:51:46] 
[00:51:46] The actual stderr differed from the expected stderr.
[00:51:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/lto-duplicate-symbols.stderr
[00:51:46] To update references, rerun the tests and pass the `--bless` flag
[00:51:46] To only update this specific test, also pass `--test-args lto-duplicate-symbols.rs`
[00:51:46] error: 1 errors occurred comparing output.
[00:51:46] status: exit code: 1
[00:51:46] status: exit code: 1
[00:51:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto-duplicate-symbols.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/auxiliary" "-A" "unused"
[00:51:46] ------------------------------------------
[00:51:46] 
[00:51:46] ------------------------------------------
[00:51:46] stderr:
[00:51:46] stderr:
[00:51:46] ------------------------------------------
[00:51:46] {"message":"Linking globals named 'foo': symbol multiply defined!","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: Linking globals named 'foo': symbol multiply defined!\n\n"}
[00:51:46] {"message":"failed to load bc of \"lto_duplicate_symbols1.3a1fbbbh-cgu.0\": ","code":null,"level":"error","spans":[],"children":[],"rendered":"error: failed to load bc of \"lto_duplicate_symbols1.3a1fbbbh-cgu.0\": \n\n"}
[00:51:46] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:46] ------------------------------------------
[00:51:46] 
[00:51:46] thread '[ui] ui/lto-duplicate-symbols.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:51:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:46] 
[00:51:46] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:51:46] 
[00:51:46] 
[00:51:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:46] 
[00:51:46] 
[00:51:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:46] Build completed unsuccessfully in 0:03:16
[00:51:46] Build completed unsuccessfully in 0:03:16
[00:51:46] make: *** [check] Error 1
[00:51:46] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:072cf746
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0939fbae
$ sudo tail -n 500 /var/log/syslog
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   3 disabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   4 disabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   5 disabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   6 disabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   7 disabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Using GB pages for direct mapping
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a9349 0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   Device   empty
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Movable zone start for each node
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Early memory node ranges
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   GE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] console [ttyS0] enabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.743820] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.751148] pid_max: default: 32768 minimum: 301
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.756088] ACPI: Core revision 20150930
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.768795] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.774388] Security Framework initialized
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.779466] Yama: becoming mindful.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.784361] AppArmor: AppArmor disabled by boot time parameter
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.791079] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.804563] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.812374] Morts 32 MCE banks
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.860437] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.863742] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.869385] Freeing SMP alternatives memory: 32K
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.881591] ftrace: allocating 32185 entries in 126 pages
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.939739] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.944168] smpboot: Max logical packages: 2
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.947266] x2apic enabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.950146] Switched APIC routing to physical x2apic.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    0.956357] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.068669] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.075655] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.087515] x86: Booting SMP configuration:
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.089854] .... node  #0, CPUs:      #1
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.092010] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.098176]  #2
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.099504] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.104901]  #3
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.106427] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.113751] x86: Booted up 1 node, 4 CPUs
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.116578] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.123047] devtmpfs: initialized
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.128868] evm: security.selinux
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.130528] evm: security.SMACK64
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.133123] evm: security.SMACK64EXEC
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.135963] evm: security.SMACK64TRANSMUTE
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.138952] evm: security.SMACK64MMAP
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.141275] evm: security.ima
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.142896] evm: security.capability
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.145800] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.152040] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.156275] pinctrl core: initialized pinctrl subsystem
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.160460] RTC time: 16:34:57, date: 08/14/18
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.164615] NET: Registered protocol family 16
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.176814] cpuidle: using governor ladder
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.188746] cpuidle: using governor menu
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.191640] PCCT header not found.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.193959] ACPI: bus type PCI registered
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.197232] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.202060] PCI: Using configuration type 1 for base access
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.218737] ACPI: Added _OSI(Module Device)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.221801] ACPI: Added _OSI(Processor Device)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.224057] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.227936] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.235081] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.264972] ACPI: Interpreter enabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.268242] ACPI: (supports S0 S3 S4 S5)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.271794] ACPI: Using IOAPIC for interrupt routing
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.275672] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.312766] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.317510] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.321040] acpi PNP0A03:00: _OSC failed (AE_3f] claimed by PIIX4 ACPI
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.431783] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.443361] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.452734] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.478908] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.490913] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.499631] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.524564] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.530612] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.537254] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.543344] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.550087] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.572656] ACPI: Enabled 16 GPEs ifff3000-0xbfffffff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.603736] NetLabel: Initializing
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.605961] NetLabel:  domain hash size = 128
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.608317] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.611778] NetLabel:  unlabeled traffic allowed by default
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.614934] amd_nb: Cannot enumerate AMD northbridges
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.618073] clocksource: Switched to clocksource kvm-clock
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.628367] pnp: PnP ACPI init
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.632619] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.632692] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.632737] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.632788] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.632833] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 16:35:09 travis-job-1.668278] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.673588] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.677108] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.682519] NET: Registered protocol family 1
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.685450] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.691038] PCI: CLS 0 bytes, default 64
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    1.691106] Unpacking initramfs...
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.796616] Freeing initrd memory: 21432K
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.800040] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.804306] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.810999] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.816971] hw unit of domain pp0-core 2^-0 Joules
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.820333] hw unit of domain package 2^-0 Joules
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.824782] hw unit of domain dram 2^-0 Joules
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.828307] Scanning for low memory corruption every 60 seconds
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.833014] audit: initializing netlink subsys (disabled)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.836805] audit: type=2000 audit(1534264499.925:1): initialized
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.840882] Initialise system trusted keyring
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.844543] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.848898] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.853384] zbud: loaded
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.855825] VFS: Disk quotas dquot_6.6.0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.857966] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.862789] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.867838] fuse init (API version 7.23)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.870608] Key type big_key registered
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.873214] Allocating IMA MOK and blacklist keyrings.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.882310] Key type asymmetric registered
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.884569] Asymmetric key parser 'x509' registered
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.888498] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.893870] io scheduler noop registered
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.896856] io scheduler deadline registered (default)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.899931] io scheduler cfq registered
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.904328] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.907843] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.912205] intel_idle: does not run on family 6 model 45
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    3.912326] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kf5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.072875] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.080776] Linux agpgart interface v0.103
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.087823] loop: module loaded
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.090166] libphy: Fixed MDIO Bus: probed
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.093567] tun: Universal TUN/TAP device driver, 1.6
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.097658] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.157474] PPP generic driver version 2.4.2
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.161530] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.165909] ehci-pci: EHCI PCI platform driver
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.169475] ehci-platform: EHCI generic platform driver
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.173676] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.178684] ohci-pci: OHCI PCI platform driver
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.182951] ohci-platform: OHCI generic platform driver
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.186948] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.192183] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.198683] i8042: Warning: Keylock active
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.203239] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.206303] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.211581] mousedev: PS/2 mouse device common for all mice
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.217844] rtc_cmos 00:00: RTC can wake from S4
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.220910] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.224344] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.229626] i2c /dev entries driver
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.232573] device-mapper: uevent: version 1.0.3
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.235622] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
22e518c1c1'
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.292349] zswap: loaded using pool lzo/zbud
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.298880] Key type trusted registered
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.310290] Key type encrypted registered
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.315121] ima: No TPM chip found, activating TPM-bypass!
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.318803] evm: HMAC attrs: 0x1
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.322123]   Magic number: 14:789:590
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.326448] rtc_cmos 00:00: setting system clock to 2018-08-14 16:35:01 UTC (1534264501)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.333959] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.337489] EDD information not available.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.341285] PM: Hibernation image not present or could not be loaded.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.342851] Freeing unused kernel memory: 1496K
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.346113] Write protecting the kernel read-only data: 14336k
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.351812] Freeing unused kernel memory: 1956K
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.355896] Freeing unused kernel memory: 92K
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.381829] systemd-udevd[119]: starting version 204
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.418136] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.460727] scsi host0: Virtio SCSI HBA
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.468222] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.480021] AVX version of gcm_enc/dec engaged.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.483161] AES CTR mode by8 optimization enabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.593785] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.593828] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.593830] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.605576] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.608838] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.609302] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.620243]  sda: sda1
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.623764] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.826258] tsc: Refined TSC clocksource calibration: 2600.000 MHz
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    4.830503] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3c3232d, max_idle_ns: 440795236700 ns
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    5.284733] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    7.446497] floppy0: no floppy controllers found
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.602129] raid6: sse2x1   gen()  8712 MB/s
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.670139] raid6: sse2x1   xor()  6484 MB/s
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.738144] raid6: sse2x2   gen() 10729 MB/s
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.806189] raid6: sse2x2   xor()  7024 MB/s
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.874149] raid6: sse2x4   gen() 12456 MB/s
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.942137] raid6: sse2x4   xor()  8682 MB/s
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.944331] raid6: using algorithm sse2x4 gen() 12456 MB/s
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.947061] raid6: .... xor() 8682 MB/s, rmw enabled
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.950010] raid6: using ssse3x2 recovery algorithm
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.955127] xor: automatically using best checksumming function:
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    8.998141]    avx       : 27448.000 MB/sec
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    9.016069] Btrfs loaded
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    9.092227] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    9.096758] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    9.205752] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    9.217006] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    9.220007] EXT4-fs (sda1): recovery complete
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [    9.229776] EXT4-fs (sda-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   11.240158] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   11.304981] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   11.388183] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   11.563590] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   11.867388] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   11.948650] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   12.047179] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   12.093719] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   12.413162] init: failsafe main process (1094) killed by TERM signal
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Running set_multiqueue.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Set channels for eth0 to 4.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 16:35:09 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-accounts: INFO Starting Google Accounts daemon.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-accounts: INFO Creating a new user account for me.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-accounts: INFO Created user account me.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-accounts: INFO Creating a new user account for aj.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-accounts: INFO Created user account aj.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-accounts: INFO Creating a new user account for carmen.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-accounts: INFO Created user account carmen.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd google-accounts: INFO Removing user packer.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   13.887928] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   13.891384] Bridge firewalling registered
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   13.907079] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   13.948304] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   14.014798] Initializing XFRM netlink socket
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   14.023284] Netfilter messages via NETLINK v0.30.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   14.026869] ctnetlink v0.93: registering with nfnetlink.
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   14.106172] floppy0: no floppy controllers found
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 16:35:10 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 16:35:11 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   14.360105] random: nonblocking pool is initialized
Aug 14 16:35:14 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd cron[1622]: (CRON) INFO (pidfile fd = 3)
Aug 14 16:35:14 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 16:35:14 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 16:35:14 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd cron[1656]: (CRON) STARTUP (fork ok)
Aug 14 16:35:14 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd cron[1656]: (CRON) INFO (Running @reboot jobs)
Aug 14 16:35:14 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd acpid: starting up with netlink and the input layer
Aug 14 16:35:14 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd acpid: 1 rule loaded
Aug 14 16:35:14 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd acpid: waiting for events: event logging is off
Aug 14 16:35:14 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd haveged: haveged starting up
Aug 14 16:35:14 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   17.426162] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1767]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: proto: precision = 0.104 usec
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: Listen normally on 3 eth0 10.20.0.64 UDP 123
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: peers refreshed
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: Listening on routing socket on fd #21 for interface updates
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [   22.640685] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd startup-script: INFO Found startup-script in metadata.
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd startup-script: INFO startup-script: job 1 at Tue Aug 14 19:45:00 2018
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd startup-script: INFO startup-script: Return code 0.
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd startup-script: INFO startup-script: Return code 0.
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd startup-script: INFO Finished running startup scripts.
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ec2: 
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ec2: #############################################################
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 16:35:19 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ec2: 1024 d4:8c:d7:bb:f7:26:02:ec:a5:50:7a:b3:71:f0:84:54  root@travis-j
Aug 14 16:38:34 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [  218.138979] cgroup: docker-runc (4774) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 16:38:34 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [  218.138982] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 16:38:35 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [  218.214710] eth0: renamed from veth624f627
Aug 14 16:38:35 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [  218.261395] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 16:38:35 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [  218.262940] docker0: port 1(veth1c2deb9) entered forwarding state
Aug 14 16:38:35 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [  218.262960] docker0: port 1(veth1c2deb9) entered forwarding state
Aug 14 16:38:35 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [  218.262989] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 16:38:38 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: Listen normally on 5 docker0 fe80::42:19ff:fe6c:ed4d UDP 123
Aug 14 16:38:38 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 14 16:38:38 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 16:38:38 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: peers refreshed
Aug 14 16:38:38 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd ntpd[1768]: new interface(s) found: waking up resolver
Aug 14 16:38:50 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [  233.317894] docker0: port 1(veth1c2deb9) entered forwarding state
Aug 14 17:17:01 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd CRON[17312]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 14 17:29:17 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [ 3260.766440] veth624f627: renamed from eth0
Aug 14 17:29:17 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [ 3260.800955] docker0: port 1(veth1c2deb9) entered disabled state
Aug 14 17:29:17 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [ 3260.832059] docker0: port 1(veth1c2deb9) entered disabled state
Aug 14 17:29:17 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [ 3260.833909] device veth1c2deb9 left promiscuous mode
Aug 14 17:29:17 travis-job-f5dfd551-10c7-459c-b658-df2a93490cbd kernel: [ 3260.833913] docker0: port 1(veth1c2deb9) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0ef12a35
