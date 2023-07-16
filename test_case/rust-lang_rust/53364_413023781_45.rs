\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/streaming_iterator.rs","byte_start":2021,"byte_end":2023,"line_start":49,"line_end":49,"column_start":48,"column_end":50,"is_primary":true,"text":[{"text":"    fn next<'a>(&'a self) -> Option<Self::Item<'a>> {","highlight_start":48,"highlight_end":50}],"label":"lifetime parameter not allowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0110]: lifetime parameters are not allowed on this type\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/streaming_iterator.rs:49:48\n   |\nLL |     fn next<'a>(&'a self) -> Option<Self::Item<'a>> {\n   |                                                ^^ lifetime parameter not allowed\n\n"}
[00:52:18] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error:= i32; // Error: shadowed lifetime
[00:52:18] 
[00:52:18] 
[00:52:18] warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18]    |
[00:52:18]    |
[00:52:18] LL |     type Bar<T>; // Error: shadowed type parameter
[00:52:18] 
[00:52:18] 
[00:52:18] warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18]    |
[00:52:18]    |
[00:52:18] LL |     type Bar<U>; // OK
[00:52:18] 
[00:52:18] 
[00:52:18] warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18]    |
[00:52:18]    |
[00:52:18] LL |     type Bar<T> = i32; // Error: shadowed type parameter
[00:52:18] 
[00:52:18] 
[00:52:18] 
[00:52:18] 
[00:52:18] 
[00:52:18] The actual stderr differed from the expected stderr.
[00:52:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/shadowing/shadowing.stderr
[00:52:18] To update references, rerun the tests and pass the `--bless` flag
[00:52:18] To only update this specific test, also pass `--test-args rfc1598-generic-associated-types/shadowing.rs`
[00:52:18] error: 1 errors occurred comparing output.
[00:52:18] status: exit code: 0
[00:52:18] status: exit code: 0
[00:52:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/shadowing/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/shadowing/auxiliary" "-A" "unused"
[00:52:18] ------------------------------------------
[00:52:18] 
[00:52:18] ------------------------------------------
[00:52:18] stderr:
[00:52:18] stderr:
[00:52:18] ------------------------------------------
[00:52:18] {"message":"generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.rs","byte_start":745,"byte_end":758,"line_start":19,"line_end":19,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    type Bar<'a>; // Error: shadowed lifetime","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.rs:19:5\n   |\nLL |     type Bar<'a>; // Error: shadowed lifetime\n   |     ^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.rs","byte_start":815,"byte_end":828,"line_start":23,"line_end":23,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    type Bar<'b>; // OK","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.rs:23:5\n   |\nLL |     type Bar<'b>; // OK\n   |     ^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.rs","byte_start":878,"byte_end":897,"line_start":27,"line_end":27,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    type Bar<'a> = i32; // Error: shadowed lifetime","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.rs:27:5\n   |\nLL |     type Bar<'a> = i32; // Error: shadowed lifetime\n   |     ^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.rs","byte_start":952,"byte_end":964,"line_start":31,"line_end":31,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    type Bar<T>; // Error: shadowed type parameter","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/shadowing.rs:31:5\n   |\nLL |     type Baexecute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:18] 
[00:52:18] 
[00:52:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:18] Build completed unsuccessfully in 0:03:12
[00:52:18] Build completed unsuccessfully in 0:03:12
[00:52:18] make: *** [check] Error 1
[00:52:18] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00c0d180
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1ee289c0
$ sudo tail -n 500 /var/log/syslog
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] SRAT: PXM 0 -> API766-418ecc52d52c kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] Zone ranges:
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   Device   empty
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] Movable zone start for each node
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] Early memory node ranges
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c ke327981] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.337232] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.341784] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.343271] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.344541] Initializing cgroup subsys io
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.345289] Initializing cgroup subsys memory
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.346221] Initializing cgroup subsys devices
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.346948] Initializing cgroup subsys freezer
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.347765] Initializing cgroup subsys net_cls
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.348441] Initializing cgroup subsys perf_event
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.349284] Initializing cgroup subsys net_prio
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.350043] Initializing cgroup subsys hugetlb
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.350762] Initializing cgroup subsys pids
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.351511] CPU: Physical Processor ID: 0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.352086] CPU: Processor Core ID: 0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.353440] mce: CPU supports 32 MCE banks
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.354307] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.355318] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.358822] Freeing SMP alternatives memory: 32K
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.368982] ftrace: allocating 32185 entries in 126 pages
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.423821] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.424944] smpboot: Max logical packages: 2
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.426127] x2apic enabled
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.427887] Switched APIC routing to physical x2apic.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.431593] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.537882] smpboot: CPU0: Intel(R) Xeon(R) C: security.SMACK64
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.565378] evm: security.SMACK64EXEC
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.566051] evm: security.SMACK64TRANSMUTE
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.566674] evm: security.SMACK64MMAP
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.567265] evm: security.ima
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.567706] evm: security.capability
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.568528] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.570104] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.571285] pinctrl core: initialized pinctrl subsystem
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.572213] RTC time: 20:40:12, date: 08/14/18
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.573765] NET: Registered protocol family 16
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.585920] cpuidle: using governor ladder
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.597929] cpuidle: using governor menu
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.598845] PCCT header not found.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.599407] ACPI: bus type PCI registered
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.600044] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.601100] PCI: Using configuration type 1 for base access
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.614857] ACPI: Added _OSI(Module Device)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.615687] ACPI: Added _OSI(Processor Device)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.616319] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.616964] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.620181] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.643287] ACPI: Interpreter enabled
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.643992] ACPI: (supports S0 S3 S4 S5)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.644578] ACPI: Using IOAPIC for interrupt routing
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.645276] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.6637-9766-418ecc52d52c kernel: [    0.761555] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.763889] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.784789] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.786123] vgaarb: loaded
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.786930] SCSI subsystem initialized
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.787626] libata version 3.00 loaded.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.787650] ACPI: bus type USB registered
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.788272] usbcore: registered new interface driver usbfs
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.789225] usbcore: registered new interface driver hub
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.790108] usbcore: registered new device driver usb
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.791087] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.792188] dmi: Firmware registration failed.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.793118] PCI: Using ACPI for IRQ routing
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.793722] PCI: pci_cache_line_size set to 64 bytes
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.793824] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.793825] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.793954] NetLabel: Initializing
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.794453] NetLabel:  domain hash size = 128
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.795072] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.795799] NetLabel:  unlabeled traffic allowed by default
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.796714] amd_nb: Cannot enumerate AMD northbridges
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.797650] clocksource: Switched to clocksource kvm-clock
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.805144] pnp: PnP ACPI init
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.805739] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.805815] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.805859] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.805914] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.805964] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.806015] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.806056] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.806229] pnp: PnP ACPI: found 7 devices
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.814393] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.815730] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.815733] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.815735] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.815736] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.815771] NET: Registered protocol family 2
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.816600] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.818477] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.819541] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.820471] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.821328] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.822950] NET: Registered protocol family 1
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.823573] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.824426] PCI: CLS 0 bytes, default 64
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    0.824472] Unpacking initramfs...
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.826281] Freeing initrd memory: 21432K
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.827474] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.828658] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.830311] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.831838] hw unit of domain pp0-core 2^-0 Joules
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.832568] hw unit of domain package 2^-0 Joules
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.833263] hw unit of domain dram 2^-16 Joules
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.833994] Scanning for low memory corruption every 60 seconds
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.835273] audit: initializing netlink subsys (disabled)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.836374] audit: type=2000 audit(1534279214.563:1): initialized
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.837769] Initialise system trusted keyring
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.838668] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.839596] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.841630] zbud: loaded
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.842265] VFS: Disk quotas dquot_6.6.0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.843361] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.844531] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.845767] fuse init (API version 7.23)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.846517] Key type big_key registered
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.847194] Allocating IMA MOK and blacklist keyrings.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.849712] Key type asymmetric registered
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.850302] Asymmetric key parser 'x509' registered
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.851030] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.852204] io scheduler noop registered
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.852854] io scheduler deadline registered (default)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.853738] io scheduler cfq registered
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.854455] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    2.855330] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 20:40:23 travi766-418ecc52d52c kernel: [    3.014394] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.016201] ohci-pci: OHCI PCI platform driver
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.017665] ohci-platform: OHCI generic platform driver
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.019477] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.021774] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.024515] i8042: Warning: Keylock active
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.026792] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.028235] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.029961] mousedev: PS/2 mouse device common for all mice
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.032009] rtc_cmos 00:00: RTC can wake from S4
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.033922] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.036110] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.038109] i2c /dev entries driver
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.039260] device-mapper: uevent: version 1.0.3
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.040748] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.043751] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.046057] NET: Registered protocol family 10
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.047900] NET: Registered protocol family 17
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.049230] Key type dns_resolver registered
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.051015] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.053070] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.054754] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.055790] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.056739] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.0598866-418ecc52d52c kernel: [    3.086934] PM: Hibernation image not present or could not be loaded.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.088240] Freeing unused kernel memory: 1496K
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.088972] Write protecting the kernel read-only data: 14336k
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.090574] Freeing unused kernel memory: 1956K
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.091689] Freeing unused kernel memory: 92K
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.104959] systemd-udevd[118]: starting version 204
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.148661] scsi host0: Virtio SCSI HBA
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.154444] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.161964] AVX2 version of gcm_enc/dec engaged.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.162740] AES CTR mode by8 optimization enabled
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.191971] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.191979] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    3.193892] sd 0:0da1): INFO: recovery required on readonly filesystem
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    7.933691] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    8.000453] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    8.006100] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    8.006864] EXT4-fs (sda1): recovery complete
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    8.011547] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    8.199150] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    8.317585] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    8.366131] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    8.521042] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [    9.008712] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d5 kernel: [   10.095746] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   10.135990] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   10.583402] init: failsafe main process (1098) killed by TERM signal
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c instance-setup: INFO Running set_multiqueue.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c instance-setup: INFO Set channels for eth0 to 4.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 20:40:23 trav account emma.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Creating a new user account for igor.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Created user account igor.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Created user account konstantinhaase.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Creating a new user account for aj.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Created user account aj.
Aug 14 20:40:23 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Creating a new user account for solarce.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Created user account solarce.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c cron[1451]: (CRON) INFO (pidfile fd = 3)
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c cron[1503]: (CRON) STARTUP (fork ok)
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c cron[1503]: (CRON) INFO (Running @reboot jobs)
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Creating a new user account for asari.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c acpid: starting up with netlink and the input layer
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c acpid: 1 rule loaded
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c acpid: waiting for events: event logging is off
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Created user account asari.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Creating a new user account for bogdana.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c haveged: haveged starting up
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Created user account bogdana.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Creating a new user account for konstantin.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   11.757090] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   11.767733] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Created user account konstantin.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Creating a new user account for carmen.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Created user account carmen.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Creating a new user account for maria.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Created user account maria.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-accounts: INFO Removing user packer.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   11.973826] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   11.976449] Bridge firewalling registered
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   11.983838] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   12.045396] Initializing XFRM netlink socket
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   12.050816] Netfilter messages via NETLINK v0.30.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   12.053234] ctnetlink v0.93: registering with nfnetlink.
Aug 14 20:40:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   12.313772] floppy0: no floppy controllers found
Aug 14 20:40:46 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpdate[1872]: adjust time server 169.254.169.254 offset 0.016999 sec
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1907]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1908]: proto: precision = 0.109 usec
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1908]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1908]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1908]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1908]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1908]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1908]: Listen normally on 3 eth0 10.20.1.176 UDP 123
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1908]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1908]: peers refreshed
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c ntpd[1908]: Listening on routing socket on fd #21 for interface updates
Aug 14 20:40:53 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   41.959357] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c startup-script: INFO Found startup-script in metadata.
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c startup-script: INFO startup-script: job 1 at Tue Aug 14 23:50:00 2018
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c startup-script: INFO startup-script: Return code 0.
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c startup-script: INFO startup-script: Return code 0.
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c startup-script: INFO Finished running startup scripts.
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c ec2: 
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c ec2: #############################################################
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c ec2: 1024 87:4b:e4:d7:a4:b8:a4:c7:04:1e:b5:c9:20:b2:08:ef  root@travis-job-320190df-e9be-4637-9766-418ecc52d52c (DSA)
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c ec2: 256 b1:6d:4d:68:ab:18:7a:c5:d4:15:c7:09:88:9f:ac:25  root@travis-job-320190df-e9be-4637-9766-418ecc52d52c (ECDSA)
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c ec2: 256 33:54:ec:c8:cd:ba:af:86:7a:b3:a9:24:7c:90:7b:fc  root@travis-job-320190df-e9be-4637-9766-418ecc52d52c (ED25519)
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c ec2: 2048 d7:cc:b2:a2:01:57:ff:06:bc:70:35:d7:d3:1a:4b:29  root@travis-job-320190df-e9be-4637-9766-418ecc52d52c (RSA)
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 20:40:54 travis-job-320190df-e9be-4637-9766-418ecc52d52c ec2: #############################################################
Aug 14 20:41:26 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [   74.450652] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 20:42:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [  132.461677] device vethe4fd57b entered promiscuous mode
Aug 14 20:42:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [  132.461731] docker0: port 1(vethe4fd57b) entered forwarding state
Aug 14 20:42:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [  132.461737] docker0: port 1(vethe4fd57b) entered forwarding state
Aug 14 20:42:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [  132.462028] docker0: port 1(vethe4fd57b) entered disabled state
Aug 14 20:42:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [  132.570121] cgroup: docker-runc (4888) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 20:42:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [  132.570124] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 20:42:24 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [  132.645736] eth0: renamed from vethe9f5758
Aug 14 2023] docker0: port 1(vethe4fd57b) entered disabled state
Aug 14 21:33:45 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [ 3213.872932] docker0: port 1(vethe4fd57b) entered disabled state
Aug 14 21:33:45 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [ 3213.874451] device vethe4fd57b left promiscuous mode
Aug 14 21:33:45 travis-job-320190df-e9be-4637-9766-418ecc52d52c kernel: [ 3213.874453] docker0: port 1(vethe4fd57b) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:011fc778
