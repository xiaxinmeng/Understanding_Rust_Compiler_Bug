plain
[00:50:23] ....................................................................................................
[00:50:26] ....................................................................................................
[00:50:28] ....................................................................................................
[00:50:31] ...................................................................................................i
[00:50:35] .............................................................................F......................
[00:50:41] ....................................................................................................
[00:50:44] .............................i......................................................................
[00:50:48] ....................................................................................................
[00:50:51] ....................................................................................................
[00:50:51] ....................................................................................................
[00:50:53] ..........................................................................i.........................
[00:50:54] ...................................
[00:50:54] failures:
[00:50:54] 
[00:50:54] ---- [ui] ui/rust-2018/uniform-paths/block-scoped-shadow.rs stdout ----
[00:50:54] diff of stderr:
[00:50:54] 
[00:50:54] 4 LL | struct std;
[00:50:54] 5    | ----------- can refer to `self::std`
[00:50:54] 6 ...
[00:50:54] - LL |     fn std() {}
[00:50:54] -    |     ----------- shadowed by block-scoped `std`
[00:50:54] 9 LL |     use std as foo;
[00:50:54] 10    |         ^^^ can refer to external crate `::std`
[00:50:54] 
[00:50:54] 
[00:50:54] The actual stderr differed from the expected stderr.
[00:50:54] The actual stderr differed from the expected stderr.
[00:50:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/block-scoped-shadow/block-scoped-shadow.stderr
[00:50:54] To update references, rerun the tests and pass the `--bless` flag
[00:50:54] To only update this specific test, also pass `--test-args rust-2018/uniform-paths/block-scoped-shadow.rs`
[00:50:54] error: 1 errors occurred comparing output.
[00:50:54] status: exit code: 1
[00:50:54] status: exit code: 1
[00:50:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow.rs" "--target=x86_64-unknown-[],"rendered":null},{"message":"relative `use` paths enabled by `#![feature(uniform_paths)]`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: `std` import is ambiguous\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow.rs:27:9\n   |\nLL | struct std;\n   | ----------- can refer to `self::std`\n...\nLL |     use std as foo;\n   |         ^^^ can refer to external crate `::std`\n   |\n   = help: write `::std` or `self::std` explicitly instead\n   = note: relative `use` paths enabled by `#![feature(uniform_paths)]`\n\n"}
[00:50:54] {"message":"`std` import is ambiguous","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow.rs","byte_start":531,"byte_end":542,"line_start":17,"line_end":17,"column_start":1,"column_end":12,"is_primary":false,"text":[{"text":"struct std;","highlight_start":1,"highlight_end":12}],"label":"can refer to `self::std`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow.rs","byte_start":655,"byte_end":666,"line_start":26,"line_end":26,"column_start":5,"column_end":16,"is_primary":false,"text":[{"text":"    fn std() {}","highlight_start":5,"highlight_end":16}],"label":"shadowed by block-scoped `std`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow.rs","byte_start":675,"byte_end":678,"line_start":27,"line_end":27,"column_start":9,"column_end":12,"is_primary":true,"texthadowed by block-scoped `Foo`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow.rs","byte_start":580,"byte_end":583,"line_start":21,"line_end":21,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"    use Foo::*;","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"write `self::Foo` explicitly instead","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"relative `use` paths enabled by `#![feature(uniform_paths)]`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: `Foo` import is ambiguous\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow.rs:21:9\n   |\nLL | enum Foo { A, B }\n   | ----------------- can refer to `self::Foo`\n...\nLL |     enum Foo {}\n   |     ----------- shadowed by block-scoped `Foo`\nLL |     use Foo::*;\n   |         ^^^\n   |\n   = help: write `self::Foo` explicitly instead\n   = note: relative `use` paths enabled by `#![feature(uniform_paths)]`\n\n"}
[00:50:54] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:50:54] ------------------------------------------
[00:50:54] 
[00:50:54] thread '[ui] ui/rust-2018/uniform-paths/block-scoped-shadow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:50:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:54] 
[00:50:54] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:50:54] 
[00:50:54] 
[00:50:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:54] 
[00:50:54] 
[00:50:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:54] Build completed unsuccessfully in 0:03:11
[00:50:54] Build completed unsuccessfully in 0:03:11
[00:50:54] Makefile:58: recipe for target 'check' failed
[00:50:54] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:086b8735
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:03cff854
$ sudo tail -n 500 /var/log/syslog
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] kvm-clock: using sched offset of 1700384699 cycles
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-431ernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] Policy zone: Normal
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] Hierarchical RCU implementation.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] console [ttyS0] enabled
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.650519] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.656785] pid_max: default: 32768 minimum: 301
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.661156] ACPI: Core revision 20150930
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.669384] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.672841] Security Framework initialized
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.675597] Yama: becoming mindful.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.677532] AppArmor: AppArmor disabled by boot time parameter
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    021:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.745932] CPU: Physical Processor ID: 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.748935] CPU: Processor Core ID: 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.752096] mce: CPU supports 32 MCE banks
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.756218] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.762706] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.770891] Freeing SMP alternatives memory: 32K
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.785540] ftrace: allocating 32185 entries in 126 pages
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.844724] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.849058] smpboot: Max logical packages: 2
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.853209] x2apic enabled
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.856499] Switched APIC routing to physical x2apic.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.862524] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.974506] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.979588] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.985252] x86: Booting SMP configuration:
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.988498] .... node  #0, CPUs:      #1
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.990690] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.997869]  #2
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    0.999366] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.005826]  #3
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.006973] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.014141] x86: Booted up 1 node, 4 CPUs
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.016527] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.021786] devtmpfs: initialized
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.027068] evm: security.selinux
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.029708] ev190440] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.194259] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.198195] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.201925] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.208539] PCI host bridge to bus 0000:00
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.210702] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.214208] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.217687] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.221468] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.225009] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.227676] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.228093] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.253830] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.278230] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.282031] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.290900] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.298206] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.318258] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.326359] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.333837] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.354414] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.359516] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    1.365144] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 16 13:21:37 travis-job-5e2bf671-081c-vis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.500113] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.501708] hw unit of domain pp0-core 2^-0 Joules
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.502450] hw unit of domain package 2^-0 Joules
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.503230] hw unit of domain dram 2^-16 Joules
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.504139] Scanning for low memory corruption every 60 seconds
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.505750] audit: initializing netlink subsys (disabled)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.506627] audit: type=2000 audit(1534425688.305:1): initialized
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.507879] Initialise system trusted keyring
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.508911] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.509966] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.512060] zbud: loaded
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.512721] VFS: Disk quotas dquot_6.6.0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.513380] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.514617] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.516047] fuse init (API version 7.23)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.516870] Key type big_key registered
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.517503] Allocating IMA MOK and blacklist keyrings.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.519339] Key type asymmetric registered
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.520093] Asymmetric key parser 'x509' registered
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.520879] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.522132] io scheduler noop registered
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.522877] io scheduler deadline registered (default)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.523648] io scheduler cfq registered
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.524331] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.525313] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.526395] intel_idle: does not run on family 6 model 63
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.526482] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.527576] ACPI: Power Button [PWRF]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.528243] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.529617] ACPI: Sleep Button [SLPF]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.530542] GHES: HEST is not enabled!
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.532747] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.533796] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.537867] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.538795] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.543785] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.566829] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.590838] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.614557] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.638009] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.642381] Linux agpgart interface v0.103
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.645452] loop: module loaded
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.646848] libphy: Fixed MDIO Bus: probed
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.648295] tun: Universal TUN/TAP device driver, 1.6
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.650113] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.681972] PPP generic driver version 2.4.2
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.683386] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.685636] ehci-pci: EHCI PCI platform driver
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.687359] ehci-platform: EHCI generic platform driver
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.689173] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.691311] ohci-pci: OHCI PCI platform driver
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.692692] ohci-platform: OHCI generic platform driver
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.694613] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.696171] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.699410] i8042: Warning: Keylock active
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.701799] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.703930] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.705869] mousedev: PS/2 mouse device common for all mice
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.708028] rtc_cmos 00:00: RTC can wake from S4
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.709954] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.711914] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kerne77] registered taskstats version 1
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.737497] Loading compiled-in X.509 certificates
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.739467] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.742590] zswap: loaded using pool lzo/zbud
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.745599] Key type trusted registered
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.749971] Key type encrypted registered
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.751319] ima: No TPM chip found, activating TPM-bypass!
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.752715] evm: HMAC attrs: 0x1
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.753988]   Magic number: 14:415:382
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.755025] acpi LNXCPU:e2: hash matches
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.756326] rtc_cmos 00:00: setting system clock to 2018-08-16 13:21:29 UTC (1534425689)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.759216] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.760812] EDD information not available.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.762251] PM: Hibernation image not present or could not be loaded.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.763599] Freeing unused kernel memory: 1496K
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.764477] Write protecting the kernel read-only data: 14336k
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.766466] Freeing unused kernel memory: 1956K
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.767521] Freeing unused kernel memory: 92K
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.780981] systemd-udevd[119]: starting version 204
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.837006] scsi host0: Virtio SCSI HBA
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.841346] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.849454] AVX2 version of gcm_enc/dec engaged.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.850692] AES CTR mode by8 optimization enabled
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.887435] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.887444] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    3.889673] sd 0:-081c-4316-9b81-150a5dba7b3e kernel: [    8.059960] raid6: sse2x1   xor()  6627 MB/s
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.127971] raid6: sse2x2   gen() 11151 MB/s
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.195962] raid6: sse2x2   xor()  7273 MB/s
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.263961] raid6: sse2x4   gen() 12908 MB/s
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.331962] raid6: sse2x4   xor()  9032 MB/s
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.399957] raid6: avx2x1   gen() 17384 MB/s
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.467960] raid6: avx2x2   gen() 21053 MB/s
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.535963] raid6: avx2x4   gen() 23191 MB/s
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.536918] raid6: using algorithm avx2x4 gen() 23191 MB/s
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.537925] raid6: using avx2x2 recovery algorithm
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.539869] xor: automatically using best checksumming function:
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.579959]    avx       : 27528.000 MB/sec
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.594073] Btrfs loaded
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.639906] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.641080] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.719689] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.727775] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.728838] EXT4-fs (sda1): recovery complete
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.734613] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    8.960238] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    9.101510] random: mountall: uninitialized urandom read (12 bytes read, 32 bits of entropy available)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    9.154022] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    9.390339] random: cloud-init: uninitialized urandom read (32 bytes read, 39 bits of entropy available)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [    9.943438] random: cloud-init: uninitialized urandom read (32 bytes read, 48 bits of entropy available)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   10.082129] systemd-udevd[703]: starting version 204
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   10.186176] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   10.256028] intel_rapl: no valid rapl domains found in package 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   10.300682] intel_rapl: no valid rapl domains found in package 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   10.315777] ppdev: user-space parallel port driver
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   10.416326] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   10.460653] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   10.522146] random: cloud-init: uninitialized urandom read (32 bytes read, 61 bits of entropy available)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   10.683545] random: cloud-init: uninitialized urandom read (32 bytes read, 61 bits of entropy available)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   10.958951] random: mktemp: uninitialized urandom read (12 bytes read, 64 bits of entropy available)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   11.026215] random: mktemp: uninitialized urandom read (6 bytes read, 65 bits of entropy available)
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   11.095359] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   11.135328] EXT4-fs (sda1): resized filesystem to 7864064
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   11.414786] init: failsafe main process (1095) killed by TERM signal
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e instance-setup: INFO Running set_multiqueue.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e instance-setup: INFO Set channels for eth0 to 4.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 16 13:21ss/net/eth0/queues/tx-0/xps_cpus
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Starting Google Accounts daemon.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   12.140990] random: nonblocking pool is initialized
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Creating a new user account for me.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Created user account me.
Aug 16 13:21:37 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Creating a new user account for henrik.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Created user account henrik.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Creating a new user account for emma.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Created user account emma.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Creating a new user account for igor.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Created user account igor.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Created user account konstantinhaase.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e cron[1403]: (CRON) INFO (pidfile fd = 3)
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e cron[1484]: (CRON) STARTUP (fork ok)
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e cron[1484]: (CRON) INFO (Running @reboot jobs)
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Creating a new user account for aj.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e acpid: starting up with netlink and the input layer
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e acpid: 1 rule loaded
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e acpid:
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Creating a new user account for konstantin.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Created user account konstantin.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Creating a new user account for carmen.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   12.851991] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   12.855098] Bridge firewalling registered
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   12.867386] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Created user account carmen.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Creating a new user account for maria.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Created user account maria.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   12.947552] Initializing XFRM netlink socket
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   12.955659] Netfilter messages via NETLINK v0.30.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   12.958088] ctnetlink v0.93: registering with nfnetlink.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e google-accounts: INFO Removing user packer.
Aug 16 13:21:38 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [   13.264119] floppy0: no floppy controllers found
Aug 16 13:22:01 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpdate[1845]: adjust time server 169.254.169.254 offset 0.015206 sec
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1850]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: proto: precision = 0.131 usec
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: Listen normally on 3 eth0 10.20.0.165 UDP 123
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: peers refreshed
Aug 16 13:22:08 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: Listening on routing socket on fd #21 for on the root
Aug 16 13:27:18 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [  353.022109] eth0: renamed from veth26e08c9
Aug 16 13:27:18 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [  353.062093] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 13:27:18 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [  353.063185] docker0: port 1(vethef3b4c8) entered forwarding state
Aug 16 13:27:18 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [  353.063204] docker0: port 1(vethef3b4c8) entered forwarding state
Aug 16 13:27:18 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [  353.063225] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 16 13:27:22 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 16 13:27:22 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: Listen normally on 6 docker0 fe80::42:31ff:fe0f:a95c UDP 123
Aug 16 13:27:22 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 16 13:27:22 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: peers refreshed
Aug 16 13:27:22 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e ntpd[1851]: new interface(s) found: waking up resolver
Aug 16 13:27:33 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [  368.109726] docker0: port 1(vethef3b4c8) entered forwarding state
Aug 16 14:15:46 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [ 3261.393084] veth26e08c9: renamed from eth0
Aug 16 14:15:46 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [ 3261.433710] docker0: port 1(vethef3b4c8) entered disabled state
Aug 16 14:15:46 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [ 3261.460603] docker0: port 1(vethef3b4c8) entered disabled state
Aug 16 14:15:46 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [ 3261.462297] device vethef3b4c8 left promiscuous mode
Aug 16 14:15:46 travis-job-5e2bf671-081c-4316-9b81-150a5dba7b3e kernel: [ 3261.462299] docker0: port 1(vethef3b4c8) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:2349da36
---
travis_time:end:1699b417:start=1534428948296419427,finish=1534428948302343709,duration=5924282
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12d3b960
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y
