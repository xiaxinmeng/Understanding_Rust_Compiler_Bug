plain
[00:56:19]     Checking rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:56:19]     Checking rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:56:19]     Checking rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:56:20]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:56:25] warning: `[Weak]` cannot be resolved, ignoring it...
[00:56:25]    --> /checkout/src/liballoc/rc.rs:809:10
[00:56:25]     |
[00:56:25] 809 |     /// [`Weak`], so we `drop` the inner value.
[00:56:25]     |
[00:56:25]     = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:56:25]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:56:25] 
[00:56:25] 
[00:56:25] warning: `[Weak]` cannot be resolved, ignoring it...
[00:56:25]     |
[00:56:25]     |
[00:56:25] 918 |     /// [`Weak`], so we `drop` the inner value.
[00:56:25]     |
[00:56:25]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:56:25] 
[00:56:25] 
[00:56:25] warning: `[Weak::upgrade]` cannot be resolved, ignoring it...
[00:56:25]     --> /checkout/src/liballoc/rc.rs:1321:33
[00:56:25]      |
[00:56:25] 1321 |     /// it. Calling [`upgrade`][Weak::upgrade] on the return value always gives [`None`].
[00:56:25]      |
[00:56:25]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:56:25] 
[00:56:25] 
[00:56:25] warning: `[Weak::upgrade]` cannot be resolved, ignoring it...
[00:56:25]      |
[00:56:25]      |
[00:56:25] 1159 |     /// Calling [`upgrade`][Weak::upgrade] on the return value always
[00:56:25]      |
[00:56:25]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:56:25] 
[00:56:25] 
[00:56:25] warning: `[Weak::upgrade]` cannot be resolved, ignoring it...
[00:56:25]     --> /checkout/src/liballoc/rc.rs:1174:29
[00:56:25]      |
[00:56:25] 1174 |     /// Calling [`upgrade`][Weak::upgrade] on the return value always gives [`None`].
[00:56:25]      |
[00:56:25]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:56:25] 
[00:56:25] 
[00:56:25] warning: `[Seek::seek_relative]` cannot be resolved, ignoring it...
[00:56:25]     |
[00:56:25]     |
[00:56:25] 297 |     /// To seek without discarding the internal buffer, use [`Seek::seek_relative`].
[00:56:25]     |
[00:56:25]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:56:25] 
[00:56:25] 
[00:56:25] warning: `[std::io::Seek]` cannot be resolved, ignoring it...
[00:56:25]     |
[00:56:25]     |
[00:56:25] 299 |     /// See [`std::io::Seek`] for more details.
[00:56:25]     |
[00:56:25]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:56:25] 
[00:56:28]     Finished release [optimized] target(s) in 49.98s
---
[01:00:04] ....................................................................................................
[01:00:07] ....................................................................................................
[01:00:10] ....................................................................................................
[01:00:13] ....................................................................................................
[01:00:16] ..............iiiiiiiii.............................................................................
[01:00:22] ....................................................................................................
[01:00:25] ....................i...............................................................................
[01:00:28] ...............................i....................................................................
[01:00:31] ....................................................................................................
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:13] 
[01:14:13] running 254 tests
[01:14:43] .....................i..............................................................................
[01:15:09] .....................i.......F......................................................................
[01:15:18] failures:
[01:15:18] 
[01:15:18] ---- [rustdoc] rustdoc/issue-29503.rs stdout ----
[01:15:18] 
[01:15:18] 
[01:15:18] error: htmldocck failed!
[01:15:18] status: exit code: 1
[01:15:18] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503" "/checkout/src/test/rustdoc/issue-29503.rs"
[01:15:18] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:15:18] ------------------------------------------
[01:15:18] 
[01:15:18] ------------------------------------------
[01:15:18] ------------------------------------------
[01:15:18] stderr:
[01:15:18] ------------------------------------------
[01:15:18] 20: @has check failed
[01:15:18]  `XPATH PATTERN` did not match
[01:15:18]  // @has - "//div[@id='implementors-list']/h3[@id='impl-MyTrait']//code" "impl<T> MyTrait for T where T: Debug"
[01:15:18] Encountered 1 errors
[01:15:18] 
[01:15:18] ------------------------------------------
[01:15:18] 
---
[01:15:18] test result: FAILED. 251 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:15:18] 
[01:15:18] 
[01:15:18] 
[01:15:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:18] 
[01:15:18] 
[01:15:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:18] Build completed unsuccessfully in 0:18:08
[01:15:18] Build completed unsuccessfully in 0:18:08
[01:15:18] Makefile:58: recipe for target 'check' failed
[01:15:18] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08b9752f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:28932372
$ sudo tail -n 500 /var/log/syslog
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Using GB pages for direct mapping
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] kvm-clock: using sched offset of 1625426456 cycles
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Zone rang6b2b144e52e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0ff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Policy zone: Normal
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Hierarchical RCU implementation.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] console [ttyS0] enabled
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.316454] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.317794] pid_max: default: 32768 minimum: 301
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.318474] ACPI: Core revision 20150930
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.324920] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.326166] Security Framework initialized
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.326733] Yama: becoming mindful.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.327310] AppArmor: AppArmor disabled by boot time parameter
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.329919] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.338980] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.344010] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.345085] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.346442] Initializing cgroup subsys io
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.347214] Initializing cgroup subsys memory
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.348181] Initializing cgroup subsys devices
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.348913] Initializing cgroup subsys freezer
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.349864] Initializing cgroup subsys net_cls
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.350499] Initializing cgroup subsys perf_event
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.351161] Initializing cgroup subsys net_prio
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.351896] Initializing cgroup subsys hugetlb
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.352508] Initializing cgroup subsys pids
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.353207] CPU: Physical Processor ID: 0
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.353817] CPU: Processor Core ID: 0
Aug 10 17:48:16 traviTRANSMUTE
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.572061] evm: security.SMACK64MMAP
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.572592] evm: security.ima
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.573135] evm: security.capability
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.574308] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.576393] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.577893] pinctrl core: initialized pinctrl subsystem
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.578993] RTC time: 17:48:06, date: 08/10/18
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.581346] NET: Registered protocol family 16
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.586943] cpuidle: using governor ladder
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.594947] cpuidle: using governor menu
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.595717] PCCT header not found.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.596336] ACPI: bus type PCI registered
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.596970] acpiphp: ACPI Hot Plug PCI Controller Driver versiond3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.674882] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.675979] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.678378] PCI host bridge to bus 0000:00
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.679184] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.680344] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.681470] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.682534] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.683830] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.684784] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.685203] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.700604] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.716052] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.717471] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.722775] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.727608] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.742988] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.748458] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.753120] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.768504] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.771164] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.773292] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.775360] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.777384] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 17:48:16 trset to 64 bytes
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.807110] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.807112] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.807229] NetLabel: Initializing
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.807775] NetLabel:  domain hash size = 128
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.808497] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.809325] NetLabel:  unlabeled traffic allowed by default
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.810370] amd_nb: Cannot enumerate AMD northbridges
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.811389] clocksource: Switched to clocksource kvm-clock
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.818858] pnp: PnP ACPI init
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.819478] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.819546] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.819596] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.819651] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.819694] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.819739] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.819781] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.819952] pnp: PnP ACPI: found 7 devices
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.827500] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.829120] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.829123] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.829125] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.829126] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.829164] NET: Registered protocol family 2
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.830059] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.831669] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.832879] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.833991] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.834953] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.835931] NET: Registered protocol family 1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.836775] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.837908] PCI: CLS 0 bytes, default 64
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    0.838667] Unpacking initramfs...
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.886190] Freeing initrd memory: 21432K
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.887122] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.888328] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [   rder 0, 4096 bytes)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.907229] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.908428] fuse init (API version 7.23)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.909391] Key type big_key registered
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.910218] Allocating IMA MOK and blacklist keyrings.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.912452] Key type asymmetric registered
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.913522] Asymmetric key parser 'x509' registered
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.914433] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.916046] io scheduler noop registered
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.916826] io scheduler deadline registered (default)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.918014] io scheduler cfq registered
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.919288] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    2.921144] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    52e kernel: [    2.988058] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.010828] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.034127] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.037278] Linux agpgart interface v0.103
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.040363] loop: module loaded
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.041288] libphy: Fixed MDIO Bus: probed
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.042258] tun: Universal TUN/TAP device driver, 1.6
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.043612] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.081750] PPP generic driver version 2.4.2
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.082633] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.084068] ehci-pci: EHCI PCI platform driver
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.085187] ehci-platform: EHCI generic platform driver
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.086326] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.087598] ohci-pci: OHCI PCI platform driver
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.088701] ohci-platform: OHCI generic platform driver
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.090173] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.091588] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.093893] i8042: Warning: Keylock active
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.095532] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.096574] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.097701] mousedev: PS/2 mouse device common for all mice
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.099070] rtc_cmos 00:00: RTC can wake from S4
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.100261] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.101491] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.102757] i2c /dev entries driver
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.103448] device-mapper: uevent: version 1.0.3
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.104392] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.106586] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.108495] NET: Registered protocol family 10
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.109604] NET: Registered protocol family 17
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.110460] Key type dns_resolver registered
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.111586] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.112613] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.113813] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.115046] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.116597] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.118799] registered taskstats version 1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.119603] Loading compiled-in X.509 certificates
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.121252] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.123238] zswap: loaded using pool lzo/zbud
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.125787] Key type trusted registered
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.129672] Key type encrypted registered
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.130514] ima: No TPM chip found, activating TPM-bypass!
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.131577] evm: HMAC attrs: 0x1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.132773]   Magic number: 14:354:845
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.133744] rtc_cmos 00:00: setting system clock to 2018-08-10 17:48:08 UTC (1533923288)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.135556] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.136585] EDD information not available.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    3.137430] PM: Hibernation image not present or could not be loaded.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  aid6: sse2x2   gen() 11245 MB/s
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.567418] raid6: sse2x2   xor()  7491 MB/s
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.635428] raid6: sse2x4   gen() 12838 MB/s
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.703423] raid6: sse2x4   xor()  9026 MB/s
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.771417] raid6: avx2x1   gen() 17116 MB/s
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.839423] raid6: avx2x2   gen() 20180 MB/s
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.907443] raid6: avx2x4   gen() 22948 MB/s
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.908669] raid6: using algorithm avx2x4 gen() 22948 MB/s
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.909470] raid6: using avx2x2 recovery algorithm
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.911649] xor: automatically using best checksumming function:
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.951440]    avx       : 27370.000 MB/sec
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    7.965724] Btrfs loaded
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    8.011328] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    8.012414] EXT4-fs (sda1): write access will be enabled during recovery
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    8.102910] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    8.114618] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    8.115493] EXT4-fs (sda1): recovery complete
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    8.119970] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    8.336136] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    8.460104] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    8.506017] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    8.711780] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    9.262983] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    9.389818] systemd-udevd[701]: starting version 204
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [    9.507851] piix4_smbus 0000:09910] EXT4-fs (sda1): resized filesystem to 7864064
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [   10.873268] init: failsafe main process (1092) killed by TERM signal
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Running set_multiqueue.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Set channels for eth0 to 4.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [   11.596082] random: nonblocking pool is initialized
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 10 17:48:16 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 17:48:17 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-accounts: INFO Starting Google Accounts daemon.
Aug 10 17:48:17 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-accounts: INFO Creating a new user account for me.
Aug 10 17:48:17 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-accounts: INFO Created user account me.
Aug 10 17:48:17 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-accounts: INFO Creating a new user account for bogdana.
Aug 10 17:48:17 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-accounts: INFO Created user account bogdana.
Aug 10 17:48:17 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-accounts: INFO Creating a new user account for aj.
Aug 10 17:48:17 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-accounts: INFO Created user account aj.
Aug 10 17:48:17 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-accounts: INFO Creating a new user account for asari.
Aug 10 17:48:17 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e google-accounts: INFO Created user account asari.
Aug 10 17:48:17 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e cron[1423]: (CRON) INFO (pidfile fd = 3)d system time with hardware clock.
Aug 10 17:48:18 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [   12.313311] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 17:48:18 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [   12.316982] Bridge firewalling registered
Aug 10 17:48:18 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [   12.328034] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 17:48:18 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [   12.391658] Initializing XFRM netlink socket
Aug 10 17:48:18 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [   12.398413] Netfilter messages via NETLINK v0.30.
Aug 10 17:48:18 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [   12.401820] ctnetlink v0.93: registering with nfnetlink.
Aug 10 17:48:18 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [   12.559490] floppy0: no floppy controllers found
Aug 10 17:48:41 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e ntpdate[1764]: adjust time server 169.254.169.254 offset -0.000620 sec
Aug 10 17:48:47 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e ntpd[1791]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 17:48:47 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e ntpd[1792]: proto: precision = 0.118 usec
Aug 10 17:48:47 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e ntpd[1792]: proto: precision = 0.118 usec
Aug 10 17:48:47 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e ntpd[1792]: ntp_io: estimated max descriptors: 1024, initial 7 entered promiscuous mode
Aug 10 18:00:07 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  721.473789] docker0: port 1(veth79797f7) entered forwarding state
Aug 10 18:00:07 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  721.473798] docker0: port 1(veth79797f7) entered forwarding state
Aug 10 18:00:07 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  721.474244] docker0: port 1(veth79797f7) entered disabled state
Aug 10 18:00:07 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  721.581397] cgroup: docker-runc (4918) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 18:00:07 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  721.581400] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 18:00:07 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  721.664907] eth0: renamed from vethc86704d
Aug 10 18:00:07 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  721.704937] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 18:00:07 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  721.706064] docker0: port 1(veth79797f7) entered forwarding state
Aug 10 18:00:07 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  721.706084] docker0: port 1(veth79797f7) entered forwarding state
Aug 10 18:00:07 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [  721.706112] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 18:00:10 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e ntpd[1792]: Listen normally on 5 docker0 fe8el: [ 4175.159849] a[19127]: segfault at 0 ip 000055f69fe33463 sp 00007ffea4e0fcc0 error 6 in a[55f69fe30000+5000]
Aug 10 18:57:51 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [ 4185.731189] a[19891]: segfault at 1 ip 0000558637f34b8c sp 00007ffd6e9bc270 error 6 in a[558637f32000+4000]
Aug 10 18:57:55 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [ 4190.094152] traps: a[20267] trap invalid opcode ip:564a8680342c sp:7ffe67c186d0 error:0 in a[564a86800000+7000]
Aug 10 19:07:37 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [ 4771.294363] vethc86704d: renamed from eth0
Aug 10 19:07:37 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [ 4771.300637] docker0: port 1(veth79797f7) entered disabled state
Aug 10 19:07:37 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [ 4771.354581] docker0: port 1(veth79797f7) entered disabled state
Aug 10 19:07:37 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [ 4771.356145] device veth79797f7 left promiscuous mode
Aug 10 19:07:37 travis-job-6d3e40c4-14f0-45c0-8783-e6b2b144e52e kernel: [ 4771.356147] docker0: port 1(veth79797f7) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1734a582
