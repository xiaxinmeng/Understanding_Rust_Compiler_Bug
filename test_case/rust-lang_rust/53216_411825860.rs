plain
[01:09:27] test [debuginfo-gdb] debuginfo/var-captured-in-stack-closure.rs ... ok
[01:09:27] 
[01:09:27] failures:
[01:09:27] 
[01:09:27] ---- [debuginfo-gdb] debuginfo/drop-locations.rs stdout ----
[01:09:27] NOTE: compiletest thinks it is using GDB without native rust support
[01:09:27] NOTE: compiletest thinks it is using GDB version 7011001
[01:09:27] 
[01:09:27] error: line not found in debugger output: [...]#loc1[...]
[01:09:27] status: exit code: 0
[01:09:27] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/drop-locations/drop-locations.debugger.script"
[01:09:27] ------------------------------------------
[01:09:27] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[01:09:27] Copyright (C) 2016 Free Software Foundation, Inc.
[01:09:27] Copyright (C) 2016 Free Software Foundation, Inc.
[01:09:27] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[01:09:27] This is free software: you are free to change and redistribute it.
[01:09:27] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[01:09:27] and "show warranty" for details.
[01:09:27] This GDB was configured as "x86_64-linux-gnu".
[01:09:27] Type "show configuration" for configuration details.
[01:09:27] For bug reporting instructions, please see:
[01:09:27] <http://www.gnu.org/software/gdb/bugs/>.
[01:09:27] Find the GDB manual and other documentation resources online at:
[01:09:27] <http://www.gnu.org/software/gdb/documentation/>.
[01:09:27] For help, type "help".
[01:09:27] Type "apropos word" to search for commands related to "word".
[01:09:27] Breakpoint 1 at 0x11c6: file /checkout/src/test/debuginfo/drop-locations.rs, line 81.
[01:09:27] [Thread debugging using libthread_db enabled]
[01:09:27] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[01:09:27] 
[01:09:27] Breakpoint 1, drop_locations::foo::h567f523ca0f8927c () at /checkout/src/test/debuginfo/drop-locations.rs:81
[01:09:27] 81         let s = String::from("s"); // #break
[01:09:27] 0x565556a0 in ?? ()
[01:09:27] #0  0x565556a0 in ?? ()
[01:09:27] ------------------------------------------
[01:09:27] stderr:
[01:09:27] ------------------------------------------
[01:09:27] ------------------------------------------
[01:09:27] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/drop-locations/drop-locations.debugger.script:11: Error in sourced command file:
[01:09:27] Cannot find bounds of current function
[01:09:27] ------------------------------------------
[01:09:27] 
[01:09:27] thread '[debuginfo-gdb] debuginfo/drop-locations.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[01:09:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:09:27] 
[01:09:27] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:09:27] 
[01:09:27] 
[01:09:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "debuginfo-gdb" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:27] 
[01:09:27] 
[01:09:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[01:09:27] Build completed unsuccessfully in 1:06:12
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0423df80
$ sudo tail -n 500 /var/log/syslog
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] kvm-clock: using sched offset of 1625574968 cycles
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Zone ranges:
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   Device   empty
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Movable zone start for each node
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Early memory node ranges
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Policy zone: Normal
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] console [ttyS0] enabled
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.451296] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.454150] pid_max: default: 32768 minimum: 301
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.455652] ACPI: Core revision 20150930
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.463805] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.466258] Security Framework initialized
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.467530] Yama: becoming mindful.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.468695] AppArmor: AppArmor disabled by boot time parameter
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.472672] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.484140] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.490468] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.493096] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.495925] Initializing cgroup subsys io
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.497523] Initializing cgroup subsys memory
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.499522] Initializing cgroup subsys devices
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.501029] Initializing cgroup subsys freezer
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.502600] Initializing cgroup subsys net_cls
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.504119] Initializing cgroup subsys perf_event
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.505751] Initializing cgroup subsys net_prio
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.507631] Initializing cgroup subsys hugetlb
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.509041] Initializing cgroup subsys pids
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.510968] CPU: Physical Processor ID: 0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.512202] CPU: Processor Core ID: 0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.515570] mce: CPU supports 32 MCE banks
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.517662] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.519398] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.523665] Freeing SMP alternatives memory: 32K
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.533796] ftrace: allocating 32185 entries in 126 pages
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.583775] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.586156] smpboot: Max logical packages: 2
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.588072] x2apic enabled
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.590320] Switched APIC routing to physical x2apic.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.595445] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.701737] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.705262] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.710315] x86: Booting SMP configuration:
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.712008] .... node  #0, CPUs:      #1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.713828] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.719303]  #2
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.720189] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.726136]  #3
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.727154] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.733167] x86: Booted up 1 node, 4 CPUs
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.734799] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.738292] devtmpfs: initialized
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.743451] evm: security.selinux
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.744912] evm: security.SMACK64
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.746043] evm: security.SMACK64EXEC
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.747646] evm: security.SMACK64TRANSMUTE
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.749307] evm: security.SMACK64MMAP
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.750438] evm: security.ima
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.751406] evm: security.capability
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.753043] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.756095] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.758670] pinctrl core: initialized pinctrl subsystem
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.760451] RTC time: 15:44:36, date: 08/09/18
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.762962] NET: Registered protocol family 16
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.773786] cpuidle: using governor ladder
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.785781] cpuidle: using governor menu
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.787062] PCCT header not found.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.788575] ACPI: bus type PCI registered
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.790133] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.792854] PCI: Using configuration type 1 for base access
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.807301] ACPI: Added _OSI(Module Device)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.809209] ACPI: Added _OSI(Processor Device)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.810996] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.812724] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.819403] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.846497] ACPI: Interpreter enabled
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.847878] ACPI: (supports S0 S3 S4 S5)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.849056] ACPI: Using IOAPIC for interrupt routing
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.850663] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.883178] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.885535] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.887906] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.890189] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.894437] PCI host bridge to bus 0000:00
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.895792] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.897774] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.900029] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.902619] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.905047] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.906966] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.907440] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.928902] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.950837] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.953483] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.961057] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.967712] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.985182] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    0.993701] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.000110] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.019962] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.023825] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.027605] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.031311] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.034760] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.057562] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.059508] vgaarb: loaded
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.060610] SCSI subsystem initialized
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.062080] libata version 3.00 loaded.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.062105] ACPI: bus type USB registered
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.063781] usbcore: registered new interface driver usbfs
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.065809] usbcore: registered new interface driver hub
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.067947] usbcore: registered new device driver usb
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.070178] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.072618] dmi: Firmware registration failed.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.074418] PCI: Using ACPI for IRQ routing
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.075837] PCI: pci_cache_line_size set to 64 bytes
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.075939] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.075941] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.076065] NetLabel: Initializing
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.077173] NetLabel:  domain hash size = 128
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.078641] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.080247] NetLabel:  unlabeled traffic allowed by default
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.082284] amd_nb: Cannot enumerate AMD northbridges
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.084248] clocksource: Switched to clocksource kvm-clock
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.093127] pnp: PnP ACPI init
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.094243] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.094307] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.094379] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.094426] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.094465] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.094503] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.094541] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.094703] pnp: PnP ACPI: found 7 devices
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.103195] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.106694] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.106697] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.106698] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.106700] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.106732] NET: Registered protocol family 2
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.108150] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.110691] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.112948] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.115102] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.116974] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.119111] NET: Registered protocol family 1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.120586] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.122735] PCI: CLS 0 bytes, default 64
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    1.123606] Unpacking initramfs...
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.294859] Freeing initrd memory: 21432K
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.296560] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.299261] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.303091] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.305716] hw unit of domain pp0-core 2^-0 Joules
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.307077] hw unit of domain package 2^-0 Joules
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.308588] hw unit of domain dram 2^-0 Joules
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.310045] Scanning for low memory corruption every 60 seconds
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.312582] audit: initializing netlink subsys (disabled)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.314432] audit: type=2000 audit(1533829478.346:1): initialized
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.317488] Initialise system trusted keyring
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.319754] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.321904] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.325124] zbud: loaded
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.326341] VFS: Disk quotas dquot_6.6.0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.327851] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.330340] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.333534] fuse init (API version 7.23)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.334912] Key type big_key registered
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.336489] Allocating IMA MOK and blacklist keyrings.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.340812] Key type asymmetric registered
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.342532] Asymmetric key parser 'x509' registered
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.344136] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.346905] io scheduler noop registered
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.348351] io scheduler deadline registered (default)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.350041] io scheduler cfq registered
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.351630] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.353423] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.355569] intel_idle: does not run on family 6 model 62
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.355686] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.358188] ACPI: Power Button [PWRF]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.359464] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.361779] ACPI: Sleep Button [SLPF]
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.363662] GHES: HEST is not enabled!
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.367614] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.369938] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.377136] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.378930] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.386380] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.409801] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.433913] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.458091] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.482538] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.486638] Linux agpgart interface v0.103
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.490687] loop: module loaded
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.492289] libphy: Fixed MDIO Bus: probed
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.493706] tun: Universal TUN/TAP device driver, 1.6
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.495586] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.537068] PPP generic driver version 2.4.2
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.538737] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.540572] ehci-pci: EHCI PCI platform driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.541870] ehci-platform: EHCI generic platform driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.543505] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.545394] ohci-pci: OHCI PCI platform driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.546757] ohci-platform: OHCI generic platform driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.548234] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.550263] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.554338] i8042: Warning: Keylock active
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.556857] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.558416] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.560468] mousedev: PS/2 mouse device common for all mice
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.562951] rtc_cmos 00:00: RTC can wake from S4
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.564791] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.566714] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.568750] i2c /dev entries driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.570125] device-mapper: uevent: version 1.0.3
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.571691] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.574531] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.578014] NET: Registered protocol family 10
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.580077] NET: Registered protocol family 17
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.582076] Key type dns_resolver registered
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.583853] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.586484] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.588187] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.589994] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.592314] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.595939] registered taskstats version 1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.597241] Loading compiled-in X.509 certificates
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.599400] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.602851] zswap: loaded using pool lzo/zbud
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.606746] Key type trusted registered
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.612868] Key type encrypted registered
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.614109] ima: No TPM chip found, activating TPM-bypass!
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.615706] evm: HMAC attrs: 0x1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.617154]   Magic number: 14:961:739
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.618290] acpi LNXCPU:68: hash matches
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.620040] acpi LNXCPU:3b: hash matches
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.621505] rtc_cmos 00:00: setting system clock to 2018-08-09 15:44:39 UTC (1533829479)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.624278] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.626086] EDD information not available.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.627376] PM: Hibernation image not present or could not be loaded.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.628896] Freeing unused kernel memory: 1496K
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.630749] Write protecting the kernel read-only data: 14336k
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.633442] Freeing unused kernel memory: 1956K
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.635452] Freeing unused kernel memory: 92K
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.651705] systemd-udevd[118]: starting version 204
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.710574] scsi host0: Virtio SCSI HBA
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.715845] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.725301] AVX version of gcm_enc/dec engaged.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.726998] AES CTR mode by8 optimization enabled
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.758928] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.759025] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.759027] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.759236] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.759238] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.759305] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.760725] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.761097]  sda: sda1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    3.762344] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    4.308352] tsc: Refined TSC clocksource calibration: 2499.814 MHz
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    4.310815] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24088a0007e, max_idle_ns: 440795263290 ns
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    4.594131] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    6.700384] floppy0: no floppy controllers found
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    7.864281] raid6: sse2x1   gen()  9188 MB/s
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    7.932290] raid6: sse2x1   xor()  7071 MB/s
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.000305] raid6: sse2x2   gen() 11387 MB/s
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.068285] raid6: sse2x2   xor()  7720 MB/s
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.136282] raid6: sse2x4   gen() 13050 MB/s
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.204278] raid6: sse2x4   xor()  9044 MB/s
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.205417] raid6: using algorithm sse2x4 gen() 13050 MB/s
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.206214] raid6: .... xor() 9044 MB/s, rmw enabled
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.206912] raid6: using ssse3x2 recovery algorithm
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.208872] xor: automatically using best checksumming function:
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.248278]    avx       : 22209.000 MB/sec
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.262007] Btrfs loaded
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.306281] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.307797] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.375291] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.386900] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.387733] EXT4-fs (sda1): recovery complete
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.392435] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.555831] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.652577] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.700783] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    8.857464] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    9.349717] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    9.485156] systemd-udevd[701]: starting version 204
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    9.587233] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    9.693220] ppdev: user-space parallel port driver
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    9.789955] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    9.842584] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [    9.911501] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   10.081763] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   10.351194] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   10.429498] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   10.510908] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   10.563772] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   11.085472] init: failsafe main process (1092) killed by TERM signal
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Running set_multiqueue.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Set channels for eth0 to 4.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Starting Google Accounts daemon.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for me.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account me.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for henrik.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account henrik.
Aug  9 15:44:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for emma.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account emma.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for igor.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account igor.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   12.229167] random: nonblocking pool is initialized
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account konstantinhaase.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for aj.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account aj.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for solarce.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account solarce.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for asari.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account asari.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for bogdana.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account bogdana.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for konstantin.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   12.477029] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   12.481031] Bridge firewalling registered
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   12.496213] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account konstantin.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   12.529923] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for carmen.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account carmen.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   12.595532] Initializing XFRM netlink socket
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Creating a new user account for maria.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   12.606429] Netfilter messages via NETLINK v0.30.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   12.609379] ctnetlink v0.93: registering with nfnetlink.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Created user account maria.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d google-accounts: INFO Removing user packer.
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   12.688312] floppy0: no floppy controllers found
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   12.688916] work still pending
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d cron[1702]: (CRON) INFO (pidfile fd = 3)
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d cron[1736]: (CRON) STARTUP (fork ok)
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d cron[1736]: (CRON) INFO (Running @reboot jobs)
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d acpid: starting up with netlink and the input layer
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d acpid: 1 rule loaded
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d acpid: waiting for events: event logging is off
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d haveged: haveged starting up
Aug  9 15:44:48 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   13.332554] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1843]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: proto: precision = 0.101 usec
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: Listen normally on 3 eth0 10.20.2.243 UDP 123
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: peers refreshed
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: Listening on routing socket on fd #21 for interface updates
Aug  9 15:44:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   18.523058] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d startup-script: INFO Found startup-script in metadata.
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d startup-script: INFO startup-script: job 1 at Thu Aug  9 18:54:00 2018
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d startup-script: INFO startup-script: Return code 0.
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d startup-script: INFO startup-script: Return code 0.
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d startup-script: INFO Finished running startup scripts.
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ec2: 
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ec2: #############################################################
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ec2: 1024 0a:a7:77:85:a8:b9:54:bf:b8:f5:60:96:b8:fc:04:cd  root@travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d (DSA)
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ec2: 256 46:25:bb:c4:79:17:1e:ae:71:31:3c:d5:43:76:fa:7a  root@travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d (ECDSA)
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ec2: 256 62:99:56:e1:97:47:2e:79:d1:c7:42:5b:5a:98:a9:43  root@travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d (ED25519)
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ec2: 2048 e3:6b:3e:02:9d:20:da:4c:55:cd:05:2f:b6:cf:c6:ea  root@travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d (RSA)
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 15:44:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ec2: #############################################################
Aug  9 15:45:03 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpdate[2250]: the NTP socket is in use, exiting
Aug  9 15:45:43 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [   68.022932] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 15:46:40 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.357382] device vethff8bf7b entered promiscuous mode
Aug  9 15:46:40 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.357442] docker0: port 1(vethff8bf7b) entered forwarding state
Aug  9 15:46:40 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.357450] docker0: port 1(vethff8bf7b) entered forwarding state
Aug  9 15:46:40 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.357835] docker0: port 1(vethff8bf7b) entered disabled state
Aug  9 15:46:40 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.461924] cgroup: docker-runc (4859) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 15:46:40 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.461927] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 15:46:40 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.553481] eth0: renamed from veth8f541fd
Aug  9 15:46:41 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.602201] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 15:46:41 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.604478] docker0: port 1(vethff8bf7b) entered forwarding state
Aug  9 15:46:41 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.604505] docker0: port 1(vethff8bf7b) entered forwarding state
Aug  9 15:46:41 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  125.604524] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 15:46:44 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: Listen normally on 5 docker0 fe80::42:fbff:fe72:2399 UDP 123
Aug  9 15:46:44 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  9 15:46:44 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 15:46:44 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: peers refreshed
Aug  9 15:46:44 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d ntpd[1844]: new interface(s) found: waking up resolver
Aug  9 15:46:56 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [  140.640944] docker0: port 1(vethff8bf7b) entered forwarding state
Aug  9 16:17:01 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d CRON[2326]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 16:35:27 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3051.939499] traps: a[11482] trap invalid opcode ip:565f822d sp:ffcd7010 error:0 in a[565f5000+6000]
Aug  9 16:35:42 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3067.204018] traps: a[14272] trap invalid opcode ip:f761be46 sp:ffaf5d10 error:0 in libstd-23f6d93d47be078e.so[f75d1000+16f000]
Aug  9 16:35:42 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3067.275942] traps: a[14285] trap invalid opcode ip:f7613e46 sp:ffc90940 error:0 in libstd-23f6d93d47be078e.so[f75c9000+16f000]
Aug  9 16:37:08 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3153.151678] traps: a[29097] trap invalid opcode ip:5664a586 sp:ff9ba4a0 error:0 in a[56648000+4000]
Aug  9 16:39:53 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3318.084595] a[25028]: segfault at 0 ip 00000000565a3ac3 sp 00000000ffca5fe0 error 6 in a[565a1000+5000]
Aug  9 16:40:02 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3326.374506] a[25791]: segfault at 1 ip 000000005659338a sp 00000000ffd98190 error 6 in a[56591000+4000]
Aug  9 16:40:06 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3330.710055] traps: a[26201] trap invalid opcode ip:565f2d55 sp:ffd38900 error:0 in a[565f0000+7000]
Aug  9 16:41:21 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3406.128339] traps: a[5553] trap invalid opcode ip:8049d6e sp:fff66320 error:0 in a[8048000+8f000]
Aug  9 16:41:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3431.706483] traps: a[8451] trap invalid opcode ip:808eb33 sp:ffafb450 error:0 in a[8048000+93000]
Aug  9 16:41:47 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3431.760460] traps: a[8458] trap invalid opcode ip:808eb33 sp:ffe9c6d0 error:0 in a[8048000+93000]
Aug  9 16:44:08 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3572.757674] traps: a[23883] trap invalid opcode ip:8049674 sp:fff80260 error:0 in a[8048000+8e000]
Aug  9 16:48:43 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3847.583189] a[20978]: segfault at 0 ip 0000000008049ae9 sp 00000000ffb892e0 error 6 in a[8048000+91000]
Aug  9 16:48:54 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3859.144742] a[21786]: segfault at 1 ip 0000000008049608 sp 00000000ffcbb700 error 6 in a[8048000+8d000]
Aug  9 16:48:59 travis-job-a185f84d-8f18-4b3d-9af9-1546b2646e8d kernel: [ 3864.276570] traps: a[22156] trap invalid opcode ip:804a056 sp:ff83cc00 error:0 in a[8048000+92000]
---
travis_time:end:289984b7:start=1533833715056154771,finish=1533833715061956996,duration=5802225
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02a1d4dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16975e1e
travis_time:start:16975e1e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0096f3d2
$ dmesg | grep -i kill
