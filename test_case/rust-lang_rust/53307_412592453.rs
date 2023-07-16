plain
[00:48:07] ....................................................................................................
[00:48:09] ....................................................................................................
[00:48:12] ....................................................................................................
[00:48:15] ....................................................................................................
[00:48:18] ................iiiiiiiii...........................................................................
[00:48:24] ....................................................................................................
[00:48:28] ......................i.............................................................................
[00:48:31] .................................i..................................................................
[00:48:34] ....................................................................................................
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:00] 
[00:57:00] running 96 tests
[00:58:52] ........................................................F..........test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:00:18] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:00:18] .F...........................
[01:00:18] 
[01:00:18] ---- [run-pass] run-pass-fulldeps/proc-macro/count_compound_ops.rs stdout ----
[01:00:18] 
[01:00:18] 
[01:00:18] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/proc-macro/auxiliary/count_compound_ops.rs" failed to compile: 
[01:00:18] status: exit code: 1
[01:00:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/auxiliary/count_compound_ops.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/count_compound_ops/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/count_compound_ops/auxiliary"
[01:00:18] ------------------------------------------
[01:00:18] 
[01:00:18] ------------------------------------------
[01:00:18] stderr:
[01:00:18] stderr:
[01:00:18] ------------------------------------------
[01:00:18] error[E0433]: failed to resolve. Could not find `core` in `{{root}}`
[01:00:18]   --> /checkout/src/test/run-pass-fulldeps/proc-macro/auxiliary/count_compound_ops.rs:22:42
[01:00:18]    |
[01:00:18] 22 |     assert_eq!(count_compound_ops_helper(quote!(++ (&&) 4@a)), 3);
[01:00:18]    |                                          ^^^^^^^^^^^^^^^^^^^ Could not find `core` in `{{root}}`
[01:00:18] error: aborting due to previous error
[01:00:18] 
[01:00:18] For more information about this error, try `rustc --explain E0433`.
[01:00:18] 
[01:00:18] 
[01:00:18] ------------------------------------------
[01:00:18] 
[01:00:18] thread '[run-pass] run-pass-fulldeps/proc-macro/count_compound_ops.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:00:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:18] 
[01:00:18] ---- [run-pass] run-pass-fulldeps/proc-macro/hygiene_example.rs stdout ----
[01:00:18] 
[01:00:18] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/proc-macro/auxiliary/hygiene_example_codegen.rs" failed to compile: 
[01:00:18] status: exit code: 1
[01:00:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/auxiliary/hygiene_example_codegen.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/hygiene_example/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/hygiene_example/auxiliary"
[01:00:18] ------------------------------------------
[01:00:18] 
[01:00:18] ------------------------------------------
[01:00:18] stderr:
[01:00:18] stderr:
[01:00:18] ------------------------------------------
[01:00:18] error[E0433]: failed to resolve. Could not find `core` in `{{root}}`
[01:00:18]   --> /checkout/src/test/run-pass-fulldeps/proc-macro/auxiliary/hygiene_example_codegen.rs:29:5
[01:00:18] 29 | /     quote! {
[01:00:18] 29 | /     quote! {
[01:00:18] 30 | |         extern crate hygiene_example; // This is never a conflict error
[01:00:18] 31 | |         let string = format!("hello {}", $input);
[01:00:18] 32 | |         //^ `format!` always resolves to the prelude macro,
[01:00:18] 33 | |         //| even if a different `format!` is in scope where `hello!` is used.
[01:00:18] 34 | |         hygiene_example::print(&string)
[01:00:18] 35 | |     }
[01:00:18]    | |_____^ Could not find `core` in `{{root}}`
[01:00:18] error: aborting due to previous error
[01:00:18] 
[01:00:18] For more information about this error, try `rustc --explain E0433`.
[01:00:18] 
---
[01:00:18] test result: FAILED. 94 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:00:18] 
[01:00:18] 
[01:00:18] 
[01:00:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:18] 
[01:00:18] 
[01:00:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:18] Build completed unsuccessfully in 0:15:00
[01:00:18] Build completed unsuccessfully in 0:15:00
[01:00:18] Makefile:58: recipe for target 'check' failed
[01:00:18] make: *** [check] Error 1

The command "stamp sh -x Mon, 13 Aug 2018 17:09:08 GMT
travis_time:end:00cbcb7d:start=1534180148384775153,finish=1534180148481459546,duration=96684393


The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:068eea1a
$ sudo tail -n 500 /var/log/syslog
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] kvm-clock: using sched offset of 1644422552 cycles
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] Zone ranges:
Augf98 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Augddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.390678] mce: CPU supports 32 MCE banks
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.391479] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.392376] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.395921] Freeing SMP alternatives memory: 32K
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.406933] ftrace: allocating 32185 entries in 126 pages
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.463636] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.464895] smpboot: Max logical packages: 2
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.466109] x2apic enabled
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.468288] Switched APIC routing to physical x2apic.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.472203] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.578283] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.580196] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.584048] x86: Booting SMP configuration:
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.584865] .... node  #0, CPUs:      #1
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.585982] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.589713]  #2
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.590199] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.594528]  #3
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.594994] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.598466] x86: Booted up 1 node, 4 CPUs
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.600597] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.603260] devtmpfs: initialized
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.608062] evm: security.selinux
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.608911] evm: security.SMACK64
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.609483] evm: security.SMACK64EXEC
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.610136] evm: security.SMACK64TRANSMUTE
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.610840] evm: security.SMACK64MMAP
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.611578] evm: security.ima
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.612140] evm: security.capability
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.613022] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.614852] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.616097] pinctrl core: initialized pinctrl subsystem
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.617287] RTC time: 16:06:51, date: 08/13/18
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.618996] NET: Registered protocol family 16
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.630324] cpuidle: using governor ladder
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.642328] cpuidle: using governor menu
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.643101] PCCT header not found.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.643999] ACPI: bus type PCI registered
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.644609] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.645725] PCI: Using configuration type 1 for base access
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.659278] ACPI: Added _OSI(Module Device)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.660163] ACPI: Added _OSI(Processor Device)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.661045] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.662036] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.665924] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.691988] ACPI: Interpreter enabled
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.692884] ACPI: (supports S0 S3 S4 S5)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.693815] ACPI: Using IOAPIC for interrupt routing
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.694845] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.727117] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.728674] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.729773] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.731189] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.734868] PCI host bridge to bus 0000:00
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.735621] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.737462] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.739586] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.740853] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.742255] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.744028] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.744484] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.763171] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.781082] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.783049] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.790143] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.796441] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.815407] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.824281] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.830674] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.850340] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.853630] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.856405] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.859758] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.862601] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.885526] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.887078] vgaarb: loaded
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.888060] SCSI subsystem initialized
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.889424] libata version 3.00 loaded.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.889451] ACPI: bus type USB registered
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.891034] usbcore: registered new interface driver usbfs
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.892122] usbcore: registered new interface driver hub
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.893278] usbcore: registered new device driver usb
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.894575] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.896052] dmi: Firmware registration failed.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.897366] PCI: Using ACPI for IRQ routing
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.898381] PCI: pci_cache_line_size set to 64 bytes
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.898496] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.898498] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.898663] NetLabel: Initializing
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.899369] NetLabel:  domain hash size = 128
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.900306] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.901496] NetLabel:  unlabeled traffic allowed by default
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.903010] amd_nb: Cannot enumerate AMD northbridges
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.904810] clocksource: Switched to clocksource kvm-clock
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.913585] pnp: PnP ACPI init
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.914241] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.914310] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.914356] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.914407] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.914450] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.914491] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.914541] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.914716] pnp: PnP ACPI: found 7 devices
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.922508] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.925206] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.925209] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.925211] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.925212] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.925253] NET: Registered protocol family 2
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.926562] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    0.929035] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug pp0-core 2^-0 Joules
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.102477] hw unit of domain package 2^-0 Joules
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.103214] hw unit of domain dram 2^-0 Joules
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.104151] Scanning for low memory corruption every 60 seconds
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.105924] audit: initializing netlink subsys (disabled)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.107175] audit: type=2000 audit(1534176413.881:1): initialized
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.108971] Initialise system trusted keyring
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.110336] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.111440] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.114705] zbud: loaded
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.115607] VFS: Disk quotas dquot_6.6.0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.117098] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.119566] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.122051] fuse init (API version 7.23)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.123074] Key type big_key registered
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.124397] Allocating IMA MOK and blacklist keyrings.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.126924] Key type asymmetric registered
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.128224] Asymmetric key parser 'x509' registered
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.129336] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.131071] io scheduler noop registered
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.132127] io scheduler deadline registered (default)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.133892] io scheduler cfq registered
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.134796] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.136721] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.138431] intel_idle: does not run on family 6 model 45
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.138585] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.476139] zswap: loaded using pool lzo/zbud
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.483941] Key type trusted registered
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.491120] Key type encrypted registered
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.493973] ima: No TPM chip found, activating TPM-bypass!
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.497781] evm: HMAC attrs: 0x1
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.500204]   Magic number: 14:749:135
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.503352] rtc_cmos 00:00: setting system clock to 2018-08-13 16:06:54 UTC (1534176414)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.508923] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.512125] EDD information not available.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.515929] PM: Hibernation image not present or could not be loaded.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.517662] Freeing unused kernel memory: 1496K
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.519956] Write protecting the kernel read-only data: 14336k
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.524620] Freeing unused kernel memory: 1956K
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.528740] Freeing unused kernel memory: 92K
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.552728] systemd-udevd[118]: starting version 204
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.597383] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.667944] scsi host0: Virtio SCSI HBA
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.683320] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.702789] AVX version of gcm_enc/dec engaged.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.708137] AES CTR mode by8 optimization enabled
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.836553] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.836791] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.836793] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.837389] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.837391] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.837474] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.840917]  sda: sda1
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    3.842995] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    4.100969] tsc: Refined TSC clocksource calibration: 2599.794 MHz
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    4.104445] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x25797970375, max_idle_ns: 440795280948 ns
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    4.467893] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    6.629047] floppy0: no floppy controllers found
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    7.808820] raid6: sse2x1   gen()  8562 MB/s
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    7.876819] raid6: sse2x1   xor()  6577 MB/s
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    7.944822] raid6: sse2x2   gen() 10962 MB/s
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.012823] raid6: sse2x2   xor()  7187 MB/s
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.080822] raid6: sse2x4   gen() 12245 MB/s
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.148829] raid6: sse2x4   xor()  8289 MB/s
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.151137] raid6: using algorithm sse2x4 gen() 12245 MB/s
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.153167] raid6: .... xor() 8289 MB/s, rmw enabled
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.155532] raid6: using ssse3x2 recovery algorithm
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.158603] xor: automatically using best checksumming function:
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.200821]    avx       : 21436.000 MB/sec
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.218682] Btrfs loaded
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.289725] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.293331] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.378233] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.387659] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.389814] EXT4-fs (sda1): recovery complete
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.397423] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.668216] random: init: uninitialized urandom read (12 bytes read, 22 bits of entropy available)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.816271] random: mountall: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    8.879092] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    9.142252] random: cloud-init: uninitialized urandom read (32 bytes read, 32 bits of entropy available)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [    9.866841] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   10.042041] systemd-udevd[701]: starting version 204
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   10.190564] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   10.245051] intel_rapl: no valid rapl domains found in package 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   10.299920] intel_rapl: no valid rapl domains found in package 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   10.308623] ppdev: user-space parallel port driver
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   10.445074] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   10.509664] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   10.584069] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   10.762187] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   11.057494] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   11.147124] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   11.244055] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   11.295812] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   11.655673] init: failsafe main process (1092) killed by TERM signal
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Running set_multiqueue.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Set channels for eth0 to 4.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 16:07:02 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 16:07:03 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 16:07:03 INFO Created user account konstantinhaase.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Creating a new user account for aj.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   13.084635] random: nonblocking pool is initialized
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Created user account aj.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Creating a new user account for solarce.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Created user account solarce.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Creating a new user account for asari.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   13.213324] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   13.218536] Bridge firewalling registered
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   13.245067] floppy0: no floppy controllers found
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   13.245228] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Created user account asari.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   13.285574] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Creating a new user account for bogdana.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Created user account bogdana.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   13.415402] Initializing XFRM netlink socket
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   13.425634] Netfilter messages via NETLINK v0.30.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [   13.429151] ctnetlink v0.93: registering with nfnetlink.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Creating a new user account for konstantin.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Created user account konstantin.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Creating a new user account for carmen.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Created user account carmen.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Creating a new user account for maria.
Aug 13 16:07:04 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 google-accounts: INFO Created user account maria.
Aug 13 16:07:04 travis-job-dddbddad-f8 startup-script: INFO startup-script: Return code 0.
Aug 13 16:07:10 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ec2: 
Aug 13 16:07:10 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ec2: 
Aug 13 16:07:10 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ec2: #############################################################
Aug 13 16:07:10 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 16:07:10 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ec2: 1024 01:0b:ea:22:09:ee:69:06:b4:4e:b7:7e:f9:49:f0:b5  root@travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 (DSA)
Aug 13 16:07:10 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ec2: 256 7f:5a:54:bf:49:8b:f1:62:b6:c3:82:92:4e:bc:ef:34  root@travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 (ECDSA)
Aug 13 16:07:10 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ec2: 256 54:63:da:5c:25:22:6a:a5:43:9e:fd:3a:c9:63:dd:d7  root@travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 (ED25519)
Aug 13 16:07:10 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ec2: 2048 9c:56:b8:81:5e:7d:9b:36:76:03:ea:57:f3:03:c8:9a  root@travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 (RSA)
Aug 13 16:07:10 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 16:07:10 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ec2: #############################################################
Aug 13 16:07:19 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ntpdate[2241]: the NTP socket is in use, exiting
Aug 13 16:08:48 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [  117.182667] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 16:09:55 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [  184.379992] device veth96dc640 entered promiscuous mode
Aug 13 16:09:55 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [  184.482691] cgroup: docker-runc (4849) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 16:09:55 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [  184.482694] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 16:09:55 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [  184.554473] eth0: renamed from vethd03807c
Aug 13 16:09:55 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [  184.595899] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 16:09:55 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [  184.597212] docker0: port 1(veth96dc640) entered forwarding state
Aug 13 16:09:55 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [  184.597233] docker0: port 1(veth96dc640) entered forwarding state
Aug 13 16:09:55 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 kernel: [  184.597264] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 16:09:59 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ntpd[1843]: Listen normally on 5 docker0 fe80::42:a4ff:feaf:e93f UDP 123
Aug 13 16:09:59 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ntpd[1843]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 13 16:09:59 travis-job-dddbddad-fa2e-4b86-bf4a-8f562e31df98 ntpd[1843]: Listen normally on 7 4067196 .
2594040 ./obj/build
1987200 ./obj/build/x86_64-unknown-linux-gnu
793028 ./src
570588 ./.git
---
154084 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
149128 ./src/llvm-emscripten/test
147708 ./obj/build/bootstrap/debug/incremental
133276 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
133272 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f3tj14zi18-1sdb9hq-1hqbd7jj2tbot
128752 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
125116 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
125112 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unkno ./src/llvm-emscripten/test/CodeGen
60840 ./src/llvm-emscripten/lib
