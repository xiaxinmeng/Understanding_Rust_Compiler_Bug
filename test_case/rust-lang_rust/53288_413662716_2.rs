\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dropck/dropck-union.rs","byte_start":1167,"byte_end":1168,"line_start":49,"line_end":49,"column_start":19,"column_end":20,"is_primary":true,"text":[{"text":"    v.0.set(Some(&v)); //~ ERROR: `v` does not live long enough","highlight_start":19,"highlight_end":20}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/dropck/dropck-union.rs","byte_start":1213,"byte_end":1214,"line_start":50,"line_end":50,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`v` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"values in a scope are dropped in the opposite order they are created","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: `v` does not live long enough\n  --> /checkout/src/test/ui/dropck/dropck-union.rs:49:19\n   |\nLL |     v.0.set(Some(&v)); //~ ERROR: `v` does not live long enough\n   |                   ^ borrowed value does not live long enough\nLL | }\n   | - `v` dropped here while still borrowed\n   |\n   = note: values in a scope are dropped in the opposite order they are created\n\n"}
[00:47:49] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:49] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:47:49] ------------------------------------------
[00:47:49] 
[00:47:49] thread '[ui] ui/dropck/dropck-union.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:47:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:49] 
[00:47:49] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:47:49] 
[00:47:49] 
[00:47:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:49] 
[00:47:49] 
[00:47:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:49] Build completed unsuccessfully in 0:03:08
[00:47:49] Build completed unsuccessfully in 0:03:08
[00:47:49] Makefile:58: recipe for target 'check' failed
[00:47:49] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18d76112
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:234747ae
$ sudo tail -n 500 /var/log/syslog
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] kvm-clock: using sched offset of 1844714415 cycles
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Zone ranges:
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   Device   empty
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Movable zone start for each node
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Early memory node ranges
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Policy zone: Normal
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  c94-1045e8088071 kernel: [    0.493637] AppArmor: AppArmor disabled by boot time parameter
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.497609] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.510000] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.516609] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.519359] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.522087] Initializing cgroup subsys io
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.523971] Initializing cgroup subsys memory
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.525224] Initializing cgroup subsys devices
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.527112] Initializing cgroup subsys freezer
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.528500] Initializing cgroup subsys net_cls
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.530137] Initializing cgroup subsys perf_event
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.531543] Initializing cgroup subsys net_prio
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.533104] Initializing cgroup subsys hugetlb
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.534730] Initializing cgroup subsys pids
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.536343] CPU: Physical Processor ID: 0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.537901] CPU: Processor Core ID: 0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.540771] mce: CPU supports 32 MCE banks
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.543170] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.545367] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.549914] Freeing SMP alternatives memory: 32K
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.562440] ftrace: allocating 32185 entries in 126 pages
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.624406] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.626858] smpboot: Max logical packages: 2
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.628820] x2apic enabled
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.632078] Switched APIC routing to physical x2apic.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.637328] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.746075] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.751216] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.757147] x86: Booting SMP configuration:
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.759413] .... node  #0, CPUs:      #1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.761364] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.768188]  #2
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.769613] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.775551]  #3
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.776658] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.783131] x86: Booted up 1 node, 4 CPUs
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.785728] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.791672] devtmpfs: initialized
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.797854] evm: security.selinux
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.800294] evm: security.SMACK64
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.802318] evm: security.SMACK64EXEC
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.804021] evm: security.SMACK64TRANSMUTE
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.806307] evm: security.SMACK64MMAP
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.808065] evm: security.ima
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.809233] evm: security.capability
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.811347] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.815035] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.818218] pinctrl core: initialized pinctrl subsystem
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.820701] RTC time: 18:55:12, date: 08/16/18
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.823786] NET: Registered protocol family 16
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.838157] cpuidle: using governor ladder
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.850143] cpuidle: using governor menu
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.852322] PCCT header not found.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.853652] ACPI: bus type PCI registered
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.855190] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.858012] PCI: Using configuration type 1 for base access
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.872431] ACPI: Added _OSI(Module Device)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.874835] ACPI: Added _OSI(Processor Device)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.876845] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.879316] ACPI: Added _OSI(Processor Aggregator Device)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.884728] ACPI: Executed 2 blocks of module-level executable AML code
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.911017] ACPI: Interpreter enabled
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.912317] ACPI: (supports S0 S3 S4 S5)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.914352] ACPI: Using IOAPIC for interrupt routing
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.916722] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.950111] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.953475] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.955845] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.957952] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.963398] PCI host bridge to bus 0000:00
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.965628] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.968054] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.971070] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.973876] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.976595] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.979007] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    0.979531] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.006287] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.033278] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.037532] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.048844] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.056928] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.081575] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.092857] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.100915] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.136531] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.141304] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.146468] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.151044] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.156466] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.179410] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.182536] vgaarb: loaded
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.184146] SCSI subsystem initialized
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.185848] libata version 3.00 loaded.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.185880] ACPI: bus type USB registered
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.188270] usbcore: registered new interface driver usbfs
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.190953] usbcore: registered new interface driver hub
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.193114] usbcore: registered new device driver usb
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.195442] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.198076] dmi: Firmware registration failed.
Aug 16 18:55:23 tra IDs PNP0303 (active)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.222927] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.222977] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.223018] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.223057] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.223112] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.223325] pnp: PnP ACPI: found 7 devices
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.232359] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.235796] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.235798] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.235800] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.235802] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.235845] NET: Registered protocol family 2
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.237564] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.240901] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.244425] TCP: Hash tables configured (established 131072 bind 65536)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.246882] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.249299] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.252577] NET: Registered protocol family 1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.254591] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.256838] PCI: CLS 0 bytes, default 64
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    1.256902] Unpacking initramfs...
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.510134] Freeing initrd memory: 21432K
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.513283] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.517811] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.522034] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.526786] hw unit of domain pp0-core 2^-0 Joules
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.528998] hw unit of domain package 2^-0 Joules
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.531006] hw unit of domain dram 2^-0 Joules
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.532771] Scanning for low memory corruption every 60 seconds
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.536586] audit: initializing netlink subsys (disabled)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.538655] audit: type=2000 audit(1534445714.337:1): initialized
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.541172] Initialise system trusted keyring
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.544411] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.547141] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.551780] zbud: loaded
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.553511] VFS: Disk quotas dquot_6.6.0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.555988] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.558751] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.561962] fuse init (API version 7.23)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.564519] Key type big_key registered
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.565959] Allocating IMA MOK and blacklist keyrings.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.572832] Key type asymmetric registered
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.574594] Asymmetric key parser 'x509' registered
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.576565] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.579484] io scheduler noop registered
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.581407] io scheduler deadline registered (default)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.583250] io scheduler cfq registered
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.584560] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.586677] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.589112] intel_idle: does not run on family 6 model 62
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.589229] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.591601] ACPI: Power Button [PWRF]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.592883] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.595363] ACPI: Sleep Button [SLPF]
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.597976] GHES: HEST is not enabled!
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.602621] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.605009] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.613537] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.615487] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.623914] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.647508] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.672746] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.697997] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.723131] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.729403] Linux agpgart interface v0.103
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.735463] loop: module loaded
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.737027] libphy: Fixed MDIO Bus: probed
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.738885] tun: Universal TUN/TAP device driver, 1.6
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.740809] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.805005] PPP generic driver version 2.4.2
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.807253] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.810086] ehci-pci: EHCI PCI platform driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.812209] ehci-platform: EHCI generic platform driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.814931] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.817423] ohci-pci: OHCI PCI platform driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.819537] ohci-platform: OHCI generic platform driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.821895] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.824867] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.828779] i8042: Warning: Keylock active
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.831688] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.834341] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.837213] mousedev: PS/2 mouse device common for all mice
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.839870] rtc_cmos 00:00: RTC can wake from S4
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.842395] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.845066] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.848112] i2c /dev entries driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.849664] device-mapper: uevent: version 1.0.3
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.852782] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.856243] ledtrig-cpu: registered to indicate activity on CPUs
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.859921] NET: Registered protocol family 10
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.862188] NET: Registered protocol family 17
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.863935] Key type dns_resolver registered
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.866238] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.868584] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.871155] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.873603] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.876152] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.880679] registered taskstats version 1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.882750] Loading compiled-in X.509 certificates
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.885638] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.890430] zswap: loaded using pool lzo/zbud
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.895735] Key type trusted registered
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.902520] Key type encrypted registered
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.904785] ima: No TPM chip found, activating TPM-bypass!
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.906766] evm: HMAC attrs: 0x1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.908515]   Magic number: 14:16:949
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.909754] mdio_bus fixed-0: hash matches
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.911662] tty tty6: hash matches
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.912932] memory memory2: hash matches
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.914659] rtc_cmos 00:00: setting system clock to 2018-08-16 18:55:15 UTC (1534445715)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.918559] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.920409] EDD information not available.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.922193] PM: Hibernation image not present or could not be loaded.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.923846] Freeing unused kernel memory: 1496K
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.925248] Write protecting the kernel read-only data: 14336k
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.928136] Freeing unused kernel memory: 1956K
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.930541] Freeing unused kernel memory: 92K
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    3.950103] systemd-udevd[119]: starting version 204
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.023844] scsi host0: Virtio SCSI HBA
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.030127] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.035539] AVX version of gcm_enc/dec engaged.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.037598] AES CTR mode by8 optimization enabled
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.044589] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.085613] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.088415] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.090913] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.093521] sd 0:0:1:0: [sda] Write Protect is off
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.095594] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.095848] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.100654]  sda: sda1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.102444] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.531280] tsc: Refined TSC clocksource calibration: 2499.794 MHz
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    4.533665] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2408774c357, max_idle_ns: 440795240686 ns
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: c94-1045e8088071 kernel: [    8.616404] Btrfs loaded
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    8.672679] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    8.675461] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    8.753563] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    8.767664] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    8.769415] EXT4-fs (sda1): recovery complete
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    8.777011] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    9.020310] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    9.161627] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    9.218278] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [    9.454453] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   10.122673] random: cloud-init: uninitialized urandom read (32 bytes read, 42 bits of entropy available)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   10.274496] systemd-udevd[703]: starting version 204
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   10.386616] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   10.493854] ppdev: user-space parallel port driver
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   10.620493] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   10.679331] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   10.748505] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   10.916141] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   11.161349] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   11.236659] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1055:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 16 18:55:23 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   12.646406] random: nonblocking pool is initialized
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Starting Google Accounts daemon.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for me.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-clock-skew: INFO Synced system time with hardware clock.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account me.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for henrik.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account henrik.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for emma.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account emma.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for igor.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 cron[1417]: (CRON) INFO (pidfile fd = 3)
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 cron[1463]: (CRON) STARTUP (fork ok)
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 cron[1463]: (CRON) INFO (Running @reboot jobs)
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account igor.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 acpid: starting up with netlink and the input layer
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 acpid: 1 rule loaded
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 acpid: waiting for events: event logging is off
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account konstantinhaase.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for aj.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 haveged: haveged starting up
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account aj.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for solarce.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   13.150154] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   13.161587] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account solarce.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for asari.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account asari.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   13.244963] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   13.249594] Bridge firewalling registered
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for bogdana.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   13.264035] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account bogdana.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for konstantin.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   13.343243] Initializing XFRM netlink socket
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account konstantin.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   13.351868] Netfilter messages via NETLINK v0.30.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   13.354755] ctnetlink v0.93: registering with nfnetlink.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for carmen.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account carmen.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Creating a new user account for maria.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 google-accounts: INFO Created user account maria.
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [   13.467539] floppy0: no floppy controllers found
Aug 16 18:55:24 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 gdge main process ended, respawning
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 startup-script: INFO Starting startup scripts.
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 startup-script: INFO Found startup-script in metadata.
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 startup-script: INFO startup-script: job 1 at Thu Aug 16 22:05:00 2018
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 startup-script: INFO startup-script: Return code 0.
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 startup-script: INFO Finished running startup scripts.
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ec2: 
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ec2: #############################################################
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ec2: 1024 72:31:0c:a9:6d:50:9d:8c:c6:90:f5:32:7e:78:56:8f  root@travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 (DSA)
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ec2: 256 d7:dd:0a:ca:c5:da:54:96:3e:97:44:bd:cc:82:e7:aa  root@travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 (ECDSA)
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ec2: 256 c4:8a:c5:a6:3d:5f:7a:f2:20:8d:76:70:bc:9a:19:7e  root@travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 (ED25519)
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ec2: 2048 e5:8c:39:b3:70:39:8a:7d:f3:eb:23:58:51:9b:ae:4f  root@travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 (RSA)
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 16 18:55:54 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ec2: #############################################################
Aug 16 18:57:17 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  126.209261] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.109233] device veth0b0582d entered promiscuous mode
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.109302] docker0: port 1(veth0b0582d) entered forwarding state
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.109311] docker0: port 1(veth0b0582d) entered forwarding state
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.109887] docker0: port 1(veth0b0582d) entered disabled state
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.236503] cgroup: docker-runc (4993) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.236506] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.328310] eth0: renamed from veth9cd9a2d
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.370509] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.371910] docker0: port 1(veth0b0582d) entered forwarding state
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.371935] docker0: port 1(veth0b0582d) entered forwarding state
Aug 16 18:59:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  230.371956] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 16 18:59:04 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ntpd[1905]: Listen normally on 5 docker0 fe80::42:acff:fea5:ec13 UDP 123
Aug 16 18:59:04 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ntpd[1905]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 16 18:59:04 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ntpd[1905]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 16 18:59:04 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ntpd[1905]: peers refreshed
Aug 16 18:59:04 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 ntpd[1905]: new interface(s) found: waking up resolver
Aug 16 18:59:16 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [  245.399620] docker0: port 1(veth0b0582d) entered forwarding state
Aug 16 19:17:01 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 CRON[13584]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 16 19:45:08 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [ 2997.360990] veth9cd9a2d: renamed from eth0
Aug 16 19:45:08 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [ 2997.382340] docker0: port 1(veth0b0582d) entered disabled state
Aug 16 19:45:08 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [ 2997.435364] docker0: port 1(veth0b0582d) entered disabled state
Aug 16 19:45:08 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [ 2997.437159] device veth0b0582d left promiscuous mode
Aug 16 19:45:08 travis-job-5a834372-c1cc-4f8e-ac94-1045e8088071 kernel: [ 2997.437162] docker0: port 1(veth0b0582d) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:05d4bdc0
---
travis_time:end:1297cd9c:start=1534448710252121701,finish=1534448710262982460,duration=10860759
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ff3b30e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0d79b974
$ dmesg | grep -i kill
