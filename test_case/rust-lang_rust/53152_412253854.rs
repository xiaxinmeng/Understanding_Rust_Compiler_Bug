plain
[01:14:45] test [debuginfo-gdb] debuginfo/var-captured-in-stack-closure.rs ... ok
[01:14:45] 
[01:14:45] failures:
[01:14:45] 
[01:14:45] ---- [debuginfo-gdb] debuginfo/drop-locations.rs stdout ----
[01:14:45] NOTE: compiletest thinks it is using GDB without native rust support
[01:14:45] NOTE: compiletest thinks it is using GDB version 7011001
[01:14:45] 
[01:14:45] error: line not found in debugger output: [...]#loc1[...]
[01:14:45] status: exit code: 0
[01:14:45] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/drop-locations/drop-locations.debugger.script"
[01:14:45] ------------------------------------------
[01:14:45] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:14:45] Copyright (C) 2016 Free Software Foundation, Inc.
[01:14:45] Copyright (C) 2016 Free Software Foundation, Inc.
[01:14:45] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:14:45] This is free software: you are free to change and redistribute it.
[01:14:45] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:14:45] and "show warranty" for details.
[01:14:45] This GDB was configured as "x86_64-linux-gnu".
[01:14:45] Type "show configuration" for configuration details.
[01:14:45] For bug reporting instructions, please see:
[01:14:45] <http://www.gnu.org/software/gdb/bugs/>.
[01:14:45] Find the GDB manual and other documentation resources online at:
[01:14:45] <http://www.gnu.org/software/gdb/documentation/>.
[01:14:45] For help, type "help".
[01:14:45] Type "apropos word" to search for commands related to "word".
[01:14:45] Breakpoint 1 at 0x11c6: file /checkout/src/test/debuginfo/drop-locations.rs, line 81.
[01:14:45] [Thread debugging using libthread_db enabled]
[01:14:45] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:14:45] 
[01:14:45] Breakpoint 1, drop_locations::foo::h567f523ca0f8927c () at /checkout/src/test/debuginfo/drop-locations.rs:81
[01:14:45] 81         let s = String::from("s"); // #break
[01:14:45] 0x565556a8 in ?? ()
[01:14:45] #0  0x565556a8 in ?? ()
[01:14:45] ------------------------------------------
[01:14:45] stderr:
[01:14:45] ------------------------------------------
[01:14:45] ------------------------------------------
[01:14:45] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/drop-locations/drop-locations.debugger.script:11: Error in sourced command file:
[01:14:45] Cannot find bounds of current function
[01:14:45] ------------------------------------------
[01:14:45] 
[01:14:45] thread '[debuginfo-gdb] debuginfo/drop-locations.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:14:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:14:45] test result: FAILED. 85 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out
[01:14:45] 
[01:14:45] 
[01:14:45] 
[01:14:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "debuginfo-gdb" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:45] 
[01:14:45] 
[01:14:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[01:14:45] Build completed unsuccessfully in 1:11:20
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:009d40d6
$ sudo tail -n 500 /var/log/syslog
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] kvm-clock: using sched offset of 1556545217 cycles
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Zone ranges:
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   Device   empty
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Movable zone start for each node
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Early memory node ranges
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Policy zone: Normal
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] console [ttyS0] enabled
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.618898] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.624041] pid_max: default: 32768 minimum: 301
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.627400] ACPI: Core revision 20150930
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.636467] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.641449] Security Framework initialized
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.644036] Yama: becoming mindful.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.645966] AppArmor: AppArmor disabled by boot time parameter
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.651375] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.663932] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.671532] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.676713] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.682716] Initializing cgroup subsys io
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.684896] Initializing cgroup subsys memory
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.687188] Initializing cgroup subsys devices
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.689786] Initializing cgroup subsys freezer
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.693236] Initializing cgroup subsys net_cls
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.695699] Initializing cgroup subsys perf_event
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.698937] Initializing cgroup subsys net_prio
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.701381] Initializing cgroup subsys hugetlb
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.703977] Initializing cgroup subsys pids
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.706826] CPU: Physical Processor ID: 0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.709244] CPU: Processor Core ID: 0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.712063] mce: CPU supports 32 MCE banks
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.714906] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.718696] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.724939] Freeing SMP alternatives memory: 32K
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.736286] ftrace: allocating 32185 entries in 126 pages
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.794182] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.798129] smpboot: Max logical packages: 2
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.800889] x2apic enabled
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.803567] Switched APIC routing to physical x2apic.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.809514] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.919705] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.924925] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.930955] x86: Booting SMP configuration:
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.932974] .... node  #0, CPUs:      #1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.935289] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.942892]  #2
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.943971] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.950137]  #3
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.951260] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.958202] x86: Booted up 1 node, 4 CPUs
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.960151] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.965732] devtmpfs: initialized
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.971036] evm: security.selinux
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.973129] evm: security.SMACK64
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.974988] evm: security.SMACK64EXEC
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.977710] evm: security.SMACK64TRANSMUTE
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.979839] evm: security.SMACK64MMAP
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.981758] evm: security.ima
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.983532] evm: security.capability
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.985690] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.991022] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.994725] pinctrl core: initialized pinctrl subsystem
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    0.997818] RTC time:  4:40:40, date: 08/11/18
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.001219] NET: Registered protocol family 16
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.015794] cpuidle: using governor ladder
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.027805] cpuidle: using governor menu
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.030198] PCCT header not found.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.032169] ACPI: bus type PCI registered
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.035057] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.038587] PCI: Using configuration type 1 for base access
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.053706] ACPI: Added _OSI(Module Device)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.056231] ACPI: Added _OSI(Processor Device)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.058772] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.061128] ACPI: Added _OSI(Processor Aggregator Device)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.066740] ACPI: Executed 2 blocks of module-level executable AML code
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.093033] ACPI: Interpreter enabled
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.095719] ACPI: (supports S0 S3 S4 S5)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.098260] ACPI: Using IOAPIC for interrupt routing
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.100781] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.135448] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.140233] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.145033] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.149285] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.157522] PCI host bridge to bus 0000:00
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.160817] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.164162] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.168375] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.172346] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.176767] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.179284] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.179719] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.206644] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.232518] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.238903] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.249594] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.258638] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.283518] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.294630] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.305310] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.330101] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.335839] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.342132] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.348028] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.353787] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.376641] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.379734] vgaarb: loaded
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.381284] SCSI subsystem initialized
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.384418] libata version 3.00 loaded.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.384444] ACPI: bus type USB registered
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.386987] usbcore: registered new interface driver usbfs
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.390445] usbcore: registered new interface driver hub
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.393625] usbcore: registered new device driver usb
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.397934] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.402325] dmi: Firmware registration failed.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.404934] PCI: Using ACPI for IRQ routing
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.408800] PCI: pci_cache_line_size set to 64 bytes
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.408940] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.408943] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.409075] NetLabel: Initializing
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.410805] NetLabel:  domain hash size = 128
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.413736] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.416908] NetLabel:  unlabeled traffic allowed by default
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.420200] amd_nb: Cannot enumerate AMD northbridges
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.423666] clocksource: Switched to clocksource kvm-clock
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.433958] pnp: PnP ACPI init
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.436326] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.436396] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.436440] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.436488] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.436529] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.436569] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.436609] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.436767] pnp: PnP ACPI: found 7 devices
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.447003] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.451721] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.451723] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.451725] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.451726] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.451766] NET: Registered protocol family 2
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.454381] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.459763] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.464581] TCP: Hash tables configured (established 131072 bind 65536)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.468550] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.473647] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.478319] NET: Registered protocol family 1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.481683] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.485160] PCI: CLS 0 bytes, default 64
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    1.485228] Unpacking initramfs...
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.576206] Freeing initrd memory: 21432K
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.578962] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.582806] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.589183] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.594046] hw unit of domain pp0-core 2^-0 Joules
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.597115] hw unit of domain package 2^-0 Joules
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.599636] hw unit of domain dram 2^-16 Joules
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.602416] Scanning for low memory corruption every 60 seconds
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.606677] audit: initializing netlink subsys (disabled)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.610459] audit: type=2000 audit(1533962442.411:1): initialized
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.615013] Initialise system trusted keyring
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.618679] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.621712] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.627172] zbud: loaded
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.628914] VFS: Disk quotas dquot_6.6.0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.631208] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.635906] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.639461] fuse init (API version 7.23)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.641909] Key type big_key registered
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.643848] Allocating IMA MOK and blacklist keyrings.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.652822] Key type asymmetric registered
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.656301] Asymmetric key parser 'x509' registered
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.658739] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.664374] io scheduler noop registered
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.666737] io scheduler deadline registered (default)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.669788] io scheduler cfq registered
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.672017] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.675350] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.678577] intel_idle: does not run on family 6 model 63
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.678690] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.683300] ACPI: Power Button [PWRF]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.685547] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.690317] ACPI: Sleep Button [SLPF]
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.693369] GHES: HEST is not enabled!
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.698631] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.702556] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.713924] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.717023] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.728540] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.754047] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.780793] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.807967] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.834849] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.842530] Linux agpgart interface v0.103
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.849013] loop: module loaded
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.851603] libphy: Fixed MDIO Bus: probed
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.854385] tun: Universal TUN/TAP device driver, 1.6
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.857761] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.914785] PPP generic driver version 2.4.2
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.918715] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.922550] ehci-pci: EHCI PCI platform driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.924990] ehci-platform: EHCI generic platform driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.928534] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.932409] ohci-pci: OHCI PCI platform driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.935719] ohci-platform: OHCI generic platform driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.938514] uhci_hcd: USB Universal Host Controller Interface driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.942319] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.946778] i8042: Warning: Keylock active
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.950435] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.952958] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.956106] mousedev: PS/2 mouse device common for all mice
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.960077] rtc_cmos 00:00: RTC can wake from S4
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.963629] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.967407] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.971482] i2c /dev entries driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.973800] device-mapper: uevent: version 1.0.3
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.976999] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.981797] ledtrig-cpu: registered to indicate activity on CPUs
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.987023] NET: Registered protocol family 10
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.990196] NET: Registered protocol family 17
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.993241] Key type dns_resolver registered
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.995787] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    3.999185] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.002848] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.007519] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.010926] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.016899] registered taskstats version 1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.018748] Loading compiled-in X.509 certificates
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.022613] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.028182] zswap: loaded using pool lzo/zbud
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.032805] Key type trusted registered
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.041454] Key type encrypted registered
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.044152] ima: No TPM chip found, activating TPM-bypass!
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.047537] evm: HMAC attrs: 0x1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.050327]   Magic number: 14:392:666
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.053038] acpi LNXCPU:15: hash matches
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.055533] rtc_cmos 00:00: setting system clock to 2018-08-11 04:40:43 UTC (1533962443)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.061349] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.064948] EDD information not available.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.067160] PM: Hibernation image not present or could not be loaded.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.068680] Freeing unused kernel memory: 1496K
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.071464] Write protecting the kernel read-only data: 14336k
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.076202] Freeing unused kernel memory: 1956K
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.079065] Freeing unused kernel memory: 92K
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.098278] systemd-udevd[119]: starting version 204
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.161186] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.169785] scsi host0: Virtio SCSI HBA
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.175728] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.190672] AVX2 version of gcm_enc/dec engaged.
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.193124] AES CTR mode by8 optimization enabled
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.262419] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.262630] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.262631] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.263249] sd 0:0:1:0: [sda] Write Protect is off
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.263252] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.263891] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.267296]  sda: sda1
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.270298] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.599796] tsc: Refined TSC clocksource calibration: 2300.000 MHz
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    4.603788] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735223b2, max_idle_ns: 440795277976 ns
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    5.006218] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    7.147905] floppy0: no floppy controllers found
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.311679] raid6: sse2x1   gen()  8833 MB/s
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.379678] raid6: sse2x1   xor()  6501 MB/s
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.447680] raid6: sse2x2   gen() 10914 MB/s
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.515679] raid6: sse2x2   xor()  6974 MB/s
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.583676] raid6: sse2x4   gen() 12619 MB/s
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.651687] raid6: sse2x4   xor()  8835 MB/s
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.719678] raid6: avx2x1   gen() 17027 MB/s
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.787676] raid6: avx2x2   gen() 20086 MB/s
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.855682] raid6: avx2x4   gen() 22467 MB/s
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.858564] raid6: using algorithm avx2x4 gen() 22467 MB/s
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.862242] raid6: using avx2x2 recovery algorithm
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.867006] xor: automatically using best checksumming function:
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.907727]    avx       : 27103.000 MB/sec
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.925445] Btrfs loaded
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.988573] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    8.992162] EXT4-fs (sda1): write access will be enabled during recovery
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    9.098242] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    9.110009] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    9.113498] EXT4-fs (sda1): recovery complete
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    9.122185] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    9.360259] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    9.508907] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    9.562926] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [    9.769472] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   10.334253] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   10.478684] systemd-udevd[702]: starting version 204
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   10.601495] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   10.649056] intel_rapl: no valid rapl domains found in package 0
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   10.712746] ppdev: user-space parallel port driver
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   10.813450] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   10.870404] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   10.940175] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   11.111010] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   11.397675] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   11.471506] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   11.557579] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   11.608068] EXT4-fs (sda1): resized filesystem to 7864064
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   12.289091] init: failsafe main process (1093) killed by TERM signal
Aug 11 04:40:51 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Running set_multiqueue.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Set channels for eth0 to 4.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-accounts: INFO Starting Google Accounts daemon.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-accounts: INFO Creating a new user account for me.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-accounts: INFO Created user account me.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-accounts: INFO Creating a new user account for aj.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-accounts: INFO Created user account aj.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-accounts: INFO Creating a new user account for carmen.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.129666] random: nonblocking pool is initialized
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-accounts: INFO Created user account carmen.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-accounts: INFO Removing user packer.
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 cron[1423]: (CRON) INFO (pidfile fd = 3)
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 cron[1459]: (CRON) STARTUP (fork ok)
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 cron[1459]: (CRON) INFO (Running @reboot jobs)
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 acpid: starting up with netlink and the input layer
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 acpid: 1 rule loaded
Aug 11 04:40:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 acpid: waiting for events: event logging is off
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 haveged: haveged starting up
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.629120] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.640063] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 google-clock-skew: INFO Synced system time with hardware clock.
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.689094] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.691603] Bridge firewalling registered
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.691720] floppy0: no floppy controllers found
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.691909] work still pending
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.700416] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.784916] Initializing XFRM netlink socket
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.791005] Netfilter messages via NETLINK v0.30.
Aug 11 04:40:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   13.793709] ctnetlink v0.93: registering with nfnetlink.
Aug 11 04:41:16 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpdate[1780]: adjust time server 169.254.169.254 offset 0.003730 sec
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1816]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: proto: precision = 0.158 usec
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: Listen normally on 3 eth0 10.20.0.26 UDP 123
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: peers refreshed
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: Listening on routing socket on fd #21 for interface updates
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   43.833308] init: plymouth-upstart-bridge main process ended, respawning
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 startup-script: INFO Found startup-script in metadata.
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 startup-script: INFO startup-script: job 1 at Sat Aug 11 07:51:00 2018
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 startup-script: INFO startup-script: Return code 0.
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 startup-script: INFO startup-script: Return code 0.
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 startup-script: INFO Finished running startup scripts.
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ec2: 
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ec2: #############################################################
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ec2: 1024 10:b1:5f:f1:7d:a8:7f:51:6f:a7:6b:43:a7:81:66:57  root@travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 (DSA)
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ec2: 256 56:03:7a:6e:ce:d7:5f:f8:83:a7:56:db:3b:55:a6:fd  root@travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 (ECDSA)
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ec2: 256 c0:8e:40:1a:0a:1c:6d:40:79:e6:cc:d0:21:50:6c:24  root@travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 (ED25519)
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ec2: 2048 0f:bc:c3:49:2b:db:23:64:10:27:2e:78:ab:1f:ac:4a  root@travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 (RSA)
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 11 04:41:23 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ec2: #############################################################
Aug 11 04:41:52 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [   72.722433] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 11 04:42:50 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [  131.230283] device veth24acfef entered promiscuous mode
Aug 11 04:42:50 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [  131.334830] cgroup: docker-runc (4810) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 11 04:42:50 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [  131.334833] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 11 04:42:50 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [  131.422990] eth0: renamed from vethb8ce851
Aug 11 04:42:50 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [  131.467877] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 11 04:42:50 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [  131.469320] docker0: port 1(veth24acfef) entered forwarding state
Aug 11 04:42:50 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [  131.469336] docker0: port 1(veth24acfef) entered forwarding state
Aug 11 04:42:50 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [  131.469362] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 11 04:42:54 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: Listen normally on 5 docker0 fe80::42:f7ff:fe35:36a0 UDP 123
Aug 11 04:42:54 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 11 04:42:54 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 11 04:42:54 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: peers refreshed
Aug 11 04:42:54 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 ntpd[1817]: new interface(s) found: waking up resolver
Aug 11 04:43:05 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [  146.494311] docker0: port 1(veth24acfef) entered forwarding state
Aug 11 05:17:01 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 CRON[2383]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 11 05:35:45 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3306.464917] traps: a[11592] trap invalid opcode ip:565ec22d sp:ffc06f50 error:0 in a[565e9000+6000]
Aug 11 05:36:02 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3322.933805] traps: a[14393] trap invalid opcode ip:f76a0e76 sp:ffb24170 error:0 in libstd-afcf0285fa903665.so[f7656000+16f000]
Aug 11 05:36:02 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3322.979512] traps: a[14400] trap invalid opcode ip:f75e0e76 sp:ff8be0e0 error:0 in libstd-afcf0285fa903665.so[f7596000+16f000]
Aug 11 05:37:34 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3415.098414] traps: a[29196] trap invalid opcode ip:56621586 sp:fff722e0 error:0 in a[5661f000+4000]
Aug 11 05:40:31 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3591.946723] a[25140]: segfault at 0 ip 00000000565feac3 sp 00000000ff91c070 error 6 in a[565fc000+5000]
Aug 11 05:40:40 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3601.179478] a[25891]: segfault at 1 ip 000000005656e38a sp 00000000ffe2d250 error 6 in a[5656c000+4000]
Aug 11 05:40:45 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3606.196031] traps: a[26339] trap invalid opcode ip:5655ad55 sp:ff925e80 error:0 in a[56558000+7000]
Aug 11 05:42:08 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3688.906023] traps: a[5694] trap invalid opcode ip:8049d6e sp:ff957840 error:0 in a[8048000+8f000]
Aug 11 05:42:35 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3715.948063] traps: a[8573] trap invalid opcode ip:808eb33 sp:ffbff570 error:0 in a[8048000+93000]
Aug 11 05:42:35 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3716.005636] traps: a[8577] trap invalid opcode ip:808eb33 sp:ffca9b00 error:0 in a[8048000+93000]
Aug 11 05:45:03 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 3864.574132] traps: a[24003] trap invalid opcode ip:8049674 sp:ffd4fff0 error:0 in a[8048000+8e000]
Aug 11 05:49:53 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 4154.354173] a[21098]: segfault at 0 ip 0000000008049ae9 sp 00000000fffa3210 error 6 in a[8048000+91000]
Aug 11 05:50:06 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 4166.685342] a[21880]: segfault at 1 ip 0000000008049608 sp 00000000ff976e70 error 6 in a[8048000+8d000]
Aug 11 05:50:11 travis-job-11e8c89d-de6b-4bfe-83bf-c0d33e354b12 kernel: [ 4172.301954] traps: a[22262] trap invalid opcode ip:804a056 sp:ffd2fac0 error:0 in a[8048000+92000]
---
travis_time:end:127bf3fa:start=1533967001414134470,finish=1533967001421069946,duration=6935476
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1dab43cb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0220be97
travis_time:start:0220be97
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0c7f4f76
$ dmesg | grep -i kill
