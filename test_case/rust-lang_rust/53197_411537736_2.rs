\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-gate.rs","byte_start":948,"byte_end":967,"line_start":29,"line_end":29,"column_start":18,"column_end":37,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"avx512bw\")]","highlight_start":18,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(avx512_target_feature)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: the target feature `avx512bw` is currently unstable (see issue #44839)\n  --> /checkout/src/test/ui/target-feature-gate.rs:29:18\n   |\nLL | #[target_feature(enable = \"avx512bw\")]\n   |                  ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(avx512_target_feature)] to the crate attributes to enable\n\n"}
[00:47:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:48] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[00:47:48] ------------------------------------------
[00:47:48] 
[00:47:48] thread '[ui] ui/target-feature-gate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:47:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:48] 
[00:47:48] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:47:48] 
[00:47:48] 
[00:47:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:48] 
[00:47:48] 
[00:47:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:47:48] Build completed unsuccessfully in 0:44:41
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0ea4cf1c
$ sudo tail -n 500 /var/log/syslog
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   1 disabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   2 disabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   3 disabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   4 disabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   5 disabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   6 disabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   7 disabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Using GB pages for direct mapping
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] kvm-clock: using sched offset of 1444935253 cycles
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Zone ranges:
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   Device   empty
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Movable zone start for each node
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Early memory node ranges
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Policy zone: Normal
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] console [ttyS0] enabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.338129] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.339500] pid_max: default: 32768 minimum: 301
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.340162] ACPI: Core revision 20150930
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.346585] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.347713] Security Framework initialized
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.348332] Yama: becoming mindful.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.349010] AppArmor: AppArmor disabled by boot time parameter
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.351549] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.360934] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.365413] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.366631] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.368209] Initializing cgroup subsys io
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.369044] Initializing cgroup subsys memory
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.369857] Initializing cgroup subsys devices
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.370804] Initializing cgroup subsys freezer
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.371664] Initializing cgroup subsys net_cls
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.372599] Initializing cgroup subsys perf_event
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.373858] Initializing cgroup subsys net_prio
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.374813] Initializing cgroup subsys hugetlb
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.375482] Initializing cgroup subsys pids
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.376598] CPU: Physical Processor ID: 0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.377558] CPU: Processor Core ID: 0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.378200] mce: CPU supports 32 MCE banks
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.379101] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.379886] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.382828] Freeing SMP alternatives memory: 32K
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.391295] ftrace: allocating 32185 entries in 126 pages
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.438492] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.439774] smpboot: Max logical packages: 2
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.440991] x2apic enabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.443099] Switched APIC routing to physical x2apic.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.446665] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.552917] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.554763] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.557038] x86: Booting SMP configuration:
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.557789] .... node  #0, CPUs:      #1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.558879] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.562482]  #2
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.563019] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.567052]  #3
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.567560] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.570918] x86: Booted up 1 node, 4 CPUs
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.571556] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.573766] devtmpfs: initialized
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.578370] evm: security.selinux
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.578882] evm: security.SMACK64
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.579436] evm: security.SMACK64EXEC
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.580074] evm: security.SMACK64TRANSMUTE
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.580693] evm: security.SMACK64MMAP
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.581298] evm: security.ima
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.581721] evm: security.capability
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.582639] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.584300] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.585430] pinctrl core: initialized pinctrl subsystem
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.586557] RTC time: 19:25:48, date: 08/08/18
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.588206] NET: Registered protocol family 16
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.595024] cpuidle: using governor ladder
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.602996] cpuidle: using governor menu
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.603865] PCCT header not found.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.604588] ACPI: bus type PCI registered
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.605543] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.606765] PCI: Using configuration type 1 for base access
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.615850] ACPI: Added _OSI(Module Device)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.617577] ACPI: Added _OSI(Processor Device)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.618256] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.619233] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.622575] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.645702] ACPI: Interpreter enabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.646502] ACPI: (supports S0 S3 S4 S5)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.647050] ACPI: Using IOAPIC for interrupt routing
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.647802] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.677466] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.679024] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.680356] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.681716] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.684645] PCI host bridge to bus 0000:00
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.685629] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.686998] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.688072] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.689307] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.690738] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.691766] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.692196] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.708013] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.722749] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.724320] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.730147] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.734164] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.747257] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.753134] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.758444] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.772669] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.775611] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.778129] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.780482] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.782692] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.802612] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.803990] vgaarb: loaded
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.804620] SCSI subsystem initialized
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.805283] libata version 3.00 loaded.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.805306] ACPI: bus type USB registered
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.806151] usbcore: registered new interface driver usbfs
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.807071] usbcore: registered new interface driver hub
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.808100] usbcore: registered new device driver usb
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.809066] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.810196] dmi: Firmware registration failed.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.811137] PCI: Using ACPI for IRQ routing
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.811882] PCI: pci_cache_line_size set to 64 bytes
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.811976] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.811978] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.812105] NetLabel: Initializing
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.812707] NetLabel:  domain hash size = 128
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.813463] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.814202] NetLabel:  unlabeled traffic allowed by default
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.815227] amd_nb: Cannot enumerate AMD northbridges
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.816201] clocksource: Switched to clocksource kvm-clock
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.823627] pnp: PnP ACPI init
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.824545] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.824624] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.824674] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.824743] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.824792] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.824837] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.824882] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.825049] pnp: PnP ACPI: found 7 devices
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.832612] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.834184] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.834187] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.834188] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.834190] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.834225] NET: Registered protocol family 2
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.835054] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.837362] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.838743] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.840325] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.841759] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.843700] NET: Registered protocol family 1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.844587] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.845778] PCI: CLS 0 bytes, default 64
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    0.845840] Unpacking initramfs...
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.842708] Freeing initrd memory: 21432K
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.843863] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.845006] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.846608] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.848516] hw unit of domain pp0-core 2^-0 Joules
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.849387] hw unit of domain package 2^-0 Joules
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.850140] hw unit of domain dram 2^-0 Joules
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.850889] Scanning for low memory corruption every 60 seconds
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.852604] audit: initializing netlink subsys (disabled)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.853428] audit: type=2000 audit(1533756350.241:1): initialized
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.854932] Initialise system trusted keyring
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.855860] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.857178] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.859484] zbud: loaded
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.860136] VFS: Disk quotas dquot_6.6.0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.861108] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.862559] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.863834] fuse init (API version 7.23)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.864684] Key type big_key registered
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.865455] Allocating IMA MOK and blacklist keyrings.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.868376] Key type asymmetric registered
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.869157] Asymmetric key parser 'x509' registered
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.870595] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.871977] io scheduler noop registered
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.872757] io scheduler deadline registered (default)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.873642] io scheduler cfq registered
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.874288] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.875236] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.876327] intel_idle: does not run on family 6 model 45
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.876424] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.877604] ACPI: Power Button [PWRF]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.878238] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.879296] ACPI: Sleep Button [SLPF]
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.880472] GHES: HEST is not enabled!
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.882775] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.883802] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.888735] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.890029] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.896115] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.918815] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.942535] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.966850] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.990099] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.992952] Linux agpgart interface v0.103
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.996099] loop: module loaded
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.997405] libphy: Fixed MDIO Bus: probed
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.998316] tun: Universal TUN/TAP device driver, 1.6
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    2.999779] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.034559] PPP generic driver version 2.4.2
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.035797] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.038203] ehci-pci: EHCI PCI platform driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.039261] ehci-platform: EHCI generic platform driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.041019] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.042850] ohci-pci: OHCI PCI platform driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.044068] ohci-platform: OHCI generic platform driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.046311] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.048257] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.051755] i8042: Warning: Keylock active
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.054254] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.055489] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.056572] mousedev: PS/2 mouse device common for all mice
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.058218] rtc_cmos 00:00: RTC can wake from S4
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.059374] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.060871] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.061975] i2c /dev entries driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.062878] device-mapper: uevent: version 1.0.3
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.064286] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.066350] ledtrig-cpu: registered to indicate activity on CPUs
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.068567] NET: Registered protocol family 10
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.070383] NET: Registered protocol family 17
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.071316] Key type dns_resolver registered
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.072345] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.073395] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.074539] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.076623] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.078208] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.080631] registered taskstats version 1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.081526] Loading compiled-in X.509 certificates
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.082984] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.085347] zswap: loaded using pool lzo/zbud
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.088056] Key type trusted registered
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.092385] Key type encrypted registered
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.093199] ima: No TPM chip found, activating TPM-bypass!
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.094346] evm: HMAC attrs: 0x1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.095551]   Magic number: 14:4:445
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.096194] acpi LNXCPU:fc: hash matches
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.097189] rtc_cmos 00:00: setting system clock to 2018-08-08 19:25:50 UTC (1533756350)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.099362] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.100639] EDD information not available.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.101249] PM: Hibernation image not present or could not be loaded.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.102644] Freeing unused kernel memory: 1496K
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.103288] Write protecting the kernel read-only data: 14336k
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.105047] Freeing unused kernel memory: 1956K
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.106117] Freeing unused kernel memory: 92K
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.119503] systemd-udevd[119]: starting version 204
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.160853] AVX version of gcm_enc/dec engaged.
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.161590] AES CTR mode by8 optimization enabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.186910] scsi host0: Virtio SCSI HBA
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.192064] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.221930] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.221942] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.224443] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.225777] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.226780] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.226905] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.230454]  sda: sda1
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.231626] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.256555] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.848346] tsc: Refined TSC clocksource calibration: 2600.000 MHz
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    3.849435] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3c3232d, max_idle_ns: 440795236700 ns
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    4.093333] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    6.168435] floppy0: no floppy controllers found
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.320230] raid6: sse2x1   gen()  9022 MB/s
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.388256] raid6: sse2x1   xor()  6742 MB/s
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.456232] raid6: sse2x2   gen() 10982 MB/s
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.524228] raid6: sse2x2   xor()  7363 MB/s
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.592261] raid6: sse2x4   gen() 12777 MB/s
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.660236] raid6: sse2x4   xor()  8967 MB/s
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.661112] raid6: using algorithm sse2x4 gen() 12777 MB/s
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.662205] raid6: .... xor() 8967 MB/s, rmw enabled
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.663143] raid6: using ssse3x2 recovery algorithm
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.665208] xor: automatically using best checksumming function:
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.704231]    avx       : 27723.000 MB/sec
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.718453] Btrfs loaded
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.755261] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.756605] EXT4-fs (sda1): write access will be enabled during recovery
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.830377] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.835928] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.836786] EXT4-fs (sda1): recovery complete
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    7.841275] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    8.031523] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    8.133816] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    8.179426] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    8.352328] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    8.864080] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    8.986853] systemd-udevd[702]: starting version 204
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.082821] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.152581] intel_rapl: no valid rapl domains found in package 0
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.202344] ppdev: user-space parallel port driver
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.288585] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.332070] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.393147] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.552956] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.829987] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.897419] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.965666] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [    9.992173] EXT4-fs (sda1): resized filesystem to 7864064
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   10.399275] init: failsafe main process (1094) killed by TERM signal
Aug  8 19:25:57 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Running set_multiqueue.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Set channels for eth0 to 4.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   11.102097] random: nonblocking pool is initialized
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-accounts: INFO Starting Google Accounts daemon.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-accounts: INFO Creating a new user account for me.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-accounts: INFO Created user account me.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-accounts: INFO Creating a new user account for bogdana.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-accounts: INFO Created user account bogdana.
Aug  8 19:25:58 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-accounts: INFO Creating a new user account for aj.
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-accounts: INFO Created user account aj.
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-accounts: INFO Creating a new user account for asari.
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-accounts: INFO Created user account asari.
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd google-accounts: INFO Removing user packer.
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd cron[1426]: (CRON) INFO (pidfile fd = 3)
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd cron[1479]: (CRON) STARTUP (fork ok)
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd cron[1479]: (CRON) INFO (Running @reboot jobs)
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd acpid: starting up with netlink and the input layer
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd acpid: 1 rule loaded
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd acpid: waiting for events: event logging is off
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd haveged: haveged starting up
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   11.649848] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   11.662370] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   11.777205] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   11.780026] Bridge firewalling registered
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   11.789904] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   11.847561] Initializing XFRM netlink socket
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   11.856056] Netfilter messages via NETLINK v0.30.
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   11.858558] ctnetlink v0.93: registering with nfnetlink.
Aug  8 19:25:59 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   12.160351] floppy0: no floppy controllers found
Aug  8 19:26:22 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpdate[1791]: adjust time server 169.254.169.254 offset 0.003980 sec
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1828]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: proto: precision = 0.102 usec
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: Listen normally on 3 eth0 10.20.0.253 UDP 123
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: peers refreshed
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: Listening on routing socket on fd #21 for interface updates
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   41.831702] init: plymouth-upstart-bridge main process ended, respawning
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd startup-script: INFO Found startup-script in metadata.
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd startup-script: INFO startup-script: job 1 at Wed Aug  8 22:36:00 2018
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd startup-script: INFO startup-script: Return code 0.
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd startup-script: INFO startup-script: Return code 0.
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd startup-script: INFO Finished running startup scripts.
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ec2: 
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ec2: #############################################################
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ec2: 1024 54:23:b3:b1:58:ee:06:e6:c5:b1:fd:5b:5d:30:20:c1  root@travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd (DSA)
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ec2: 256 ec:d5:c7:11:ba:a5:4f:bd:48:e2:40:60:44:5a:c5:d8  root@travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd (ECDSA)
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ec2: 256 7e:ef:38:e3:85:d6:62:97:39:b9:42:15:1f:f2:b0:6d  root@travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd (ED25519)
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ec2: 2048 eb:09:0b:89:33:91:7b:b1:8e:95:06:0a:51:c8:a3:da  root@travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd (RSA)
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 19:26:29 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ec2: #############################################################
Aug  8 19:26:56 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [   68.571670] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 19:27:51 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [  123.626432] device vethcf3a3f1 entered promiscuous mode
Aug  8 19:27:51 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [  123.723085] cgroup: docker-runc (4825) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 19:27:51 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [  123.723088] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 19:27:51 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [  123.791444] eth0: renamed from vethf2a79db
Aug  8 19:27:51 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [  123.832642] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 19:27:51 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [  123.833680] docker0: port 1(vethcf3a3f1) entered forwarding state
Aug  8 19:27:51 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [  123.833697] docker0: port 1(vethcf3a3f1) entered forwarding state
Aug  8 19:27:51 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [  123.833719] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 19:27:54 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  8 19:27:54 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: Listen normally on 6 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 19:27:54 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: Listen normally on 7 docker0 fe80::42:7cff:feb7:482d UDP 123
Aug  8 19:27:54 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: peers refreshed
Aug  8 19:27:54 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd ntpd[1829]: new interface(s) found: waking up resolver
Aug  8 19:28:06 travis-job-052329ae-8aa3-40a2-ad92-585c75d4cafd kernel: [  138.867348] docker0: port 1(vethcf3a3f1) entered forwarding state
---
travis_time:end:267a59b1:start=1533759286465596349,finish=1533759286470462958,duration=4866609
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00489fba
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:037d2e6c
travis_time:start:037d2e6c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:039fa47c
$ dmesg | grep -i kill
