\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/book/first-edition/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/std-uncopyable-atomics.rs","byte_start":779,"byte_end":782,"line_start":23,"line_end":23,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"    let x = *&x; //~ ERROR: cannot move out of borrowed content","highlight_start":13,"highlight_end":16}],"label":"cannot move out of borrowed content","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider removing the `*`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/std-uncopyable-atomics.rs","byte_start":779,"byte_end":782,"line_start":23,"line_end":23,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"    let x = *&x; //~ ERROR: cannot move out of borrowed content","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"&x","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0507]: cannot move out of borrowed content\n  --> /checkout/src/test/ui/std-uncopyable-atomics.rs:23:13\n   |\nLL |     let x = *&x; //~ ERROR: cannot move out of borrowed content\n   |             ^^^\n   |             |\n   |             cannot move out of borrowed content\n   |             hend":909,"line_start":25,"line_end":25,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"    let x = *&x; //~ ERROR: cannot move out of borrowed content","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"&x","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0507]: cannot move out of borrowed content\n  --> /checkout/src/test/ui/std-uncopyable-atomics.rs:25:13\n   |\nLL |     let x = *&x; //~ ERROR: cannot move out of borrowed content\n   |             ^^^\n   |             |\n   |             cannot move out of borrowed content\n   |             help: consider removing the `*`: `&x`\n\n"}
[00:52:46] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:52:46] {"message":"For more information about this error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0507`.\n"}
[00:52:46] ------------------------------------------
[00:52:46] 
[00:52:46] thread '[ui (nll)] ui/std-uncopyable-atomics.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:52:46] 
---
[00:52:46] test result: FAILED. 4039 passed; 6 failed; 85 ignored; 0 measured; 0 filtered out
[00:52:46] 
[00:52:46] 
[00:52:46] 
[00:52:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:52:46] 
[00:52:46] 
[00:52:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:46] Build completed unsuccessfully in 0:05:10
[00:52:46] Build completed unsuccessfully in 0:05:10
[00:52:46] Makefile:58: recipe for target 'check' failed
[00:52:46] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21fe291b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:06485a18
$ sudo tail -n 500 /var/log/syslog
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d470505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] kvm-clock: using sched offset of 1523745780 cycles
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Zone ranges:
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   Device   empty
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Movable zone start for each node
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Early memory node ranges
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-438b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Policy zone: Normal
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] console [ttyS0] enabled
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.000000] tsc: Detected 2299.998 MHz processor
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.327402] Calibrating delay loop (skipped) preset value.. 4599.99 BogoMIPS (lpj=9199992)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.328682] pid_max: default: 32768 minimum: 301
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.329331] ACPI: Core revision 20150930
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.335695] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.336764] Security Framework initialized
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.337376] Yama: becoming mindful.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.337985] AppArmor: AppArmor disabled by boot time parameter
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.340493] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.349487] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.353720] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.354727] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.355928] Initializing cgroup subsys io
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.356522] Initializing cgroup subsys memory
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.357137] Initializing cgroup subsys devices
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.357775] Initializing cgroup subsys freezer
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.358423] Initializing cgroup subsys net_cls
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.359143] Initializing cgroup subsys perf_event
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.359800] Initializing cgroup subsys net_prio
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.360425] Initializing cgroup subsys hugetlb
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.361069] Initializing cgroup subsys pids
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.361760] CPU: Physical Processor ID: 0
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.362318] CPU: Processor Core ID: 0
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.363686] mce: CPU supports 32 MCE banks
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.364409] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.365370] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.368688] Freeing SMP alternatives memory: 32K
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.378196] ftrace: allocating 32185 entries in 126 pages
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.432104] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.433108] smpboot: Max logical packages: 2
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.434203] x2apic enabled
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.435779] Switched Ael: [    0.594602] cpuidle: using governor ladder
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.606604] cpuidle: using governor menu
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.607182] PCCT header not found.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.607880] ACPI: bus type PCI registered
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.608867] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.610310] PCI: Using configuration type 1 for base access
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.623723] ACPI: Added _OSI(Module Device)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.624653] ACPI: Added _OSI(Processor Device)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.625321] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.626007] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.629430] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.652163] ACPI: Interpreter enabled
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.652817] ACPI: (supports S0 S3 S4 S5)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.653389] ACPI: Using IOAPIC for interrupt routing
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.654119] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.683515] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.684477] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.685461] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.686409] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.688635] PCI host bridge to bus 0000:00
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.689426] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.690337] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.691451] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.692482] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.693593] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.694399] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.694802] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.708164] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.720394] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.721760] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.727289] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.731331] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.742874] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.747708] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.751661] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.763403] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 19:55:16 trav4 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.798907] dmi: Firmware registration failed.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.799681] PCI: Using ACPI for IRQ routing
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.800263] PCI: pci_cache_line_size set to 64 bytes
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.800382] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.800384] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.800504] NetLabel: Initializing
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.800986] NetLabel:  domain hash size = 128
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.801627] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.802333] NetLabel:  unlabeled traffic allowed by default
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.803333] amd_nb: Cannot enumerate AMD northbridges
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.804134] clocksource: Switched to clocksource kvm-clock
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.811167] pnp: PnP ACPI init
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.811814] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.811891] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.811943] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.811995] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.812038] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.812084] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.812126] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.812298] pnp: PnP ACPI: found 7 devices
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.820551] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.821777] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.821779] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.821781] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.821782] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.821815] NET: Registered protocol family 2
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.822765] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.824717] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.825738] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.826688] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.827563] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.828628] NET: Registered protocol family 1
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.829250] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.830153] PCI: CLS 0 bytes, default 64
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    0.830854] Unpacking initramfs...
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.862988] Freeing initrd memory: 21432K
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.878251] zbud: loaded
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.878958] VFS: Disk quotas dquot_6.6.0
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.879545] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.881019] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.882527] fuse init (API version 7.23)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.883317] Key type big_key registered
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.883896] Allocating IMA MOK and blacklist keyrings.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.885675] Key type asymmetric registered
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.886259] Asymmetric key parser 'x509' registered
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.887166] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.888339] io scheduler noop registered
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.888939] io scheduler deadline registered (default)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.889772] io scheduler cfq registered
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.890396] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.891178] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.892115] intel_idle: does not run on family 6 model 63
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.892223] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.893319] ACPI: Power Button [PWRF]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.893970] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.895125] ACPI: Sleep Button [SLPF]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.896056] GHES: HEST is not enabled!
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.898203] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.899138] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.902872] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.903852] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.907721] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.930103] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.952736] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.975710] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    2.998663] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.001322] Linux agpgart interface v0.103
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.004400] loop: module loaded
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.005705] libphy: Fixed MDIO Bus: probed
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.007019] tun: Universal TUN/TAP device driver, 1.6
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.008536] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.038473] PPP generic driver version 2.4.2
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.040413] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.042912] ehci-pci: EHCI PCI platform driver
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.044471] ehci-platform: EHCI generic platform driver
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.046045] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.048102] ohci-pci: OHCI PCI platform driver
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.049582] ohci-platform: OHCI generic platform driver
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.051238] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.053443] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.056364] i8042: Warning: Keylock active
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.058527] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.060310] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.062046] mousedev: PS/2 mouse device common for all mice
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.064181] rtc_cmos 00:00: RTC can wake from S4
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d470393 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.090061] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.093273] registered taskstats version 1
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.094623] Loading compiled-in X.509 certificates
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.096794] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.099656] zswap: loaded using pool lzo/zbud
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.102905] Key type trusted registered
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.107548] Key type encrypted registered
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.108262] ima: No TPM chip found, activating TPM-bypass!
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.109140] evm: HMAC attrs: 0x1
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.110098]   Magic number: 14:969:950
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.111412] tty tty7: hash matches
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.112680] memory memory3: hash matches
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.114015] rtc_cmos 00:00: setting system clock to 2018-08-14 19:55:08 UTC (1534276508)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.116630] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.118574] EDD information not available.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.120056] PM: Hibernation image not present or could not be loaded.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.121410] Freeing unused kernel memory: 1496K
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.122099] Write protecting the kernel read-only data: 14336k
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.123786] Freeing unused kernel memory: 1956K
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.124791] Freeing unused kernel memory: 92K
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.138094] systemd-udevd[118]: starting version 204
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.186924] scsi host0: Virtio SCSI HBA
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.189524] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.197579] AVX2 version of gcm_enc/dec engaged.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.198358] AES CTR mode by8 optimization enabled
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.226013] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.226028] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.226029] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.226144] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.226145] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.226175] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.227359]  sda: sda1
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.227893] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.264687] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    3.868393] tsc: Refined TSC clocksource calibration: 2299.999 MHz
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6B/s
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    7.881722] raid6: using avx2x2 recovery algorithm
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    7.883771] xor: automatically using best checksumming function:
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    7.924144]    avx       : 27337.000 MB/sec
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    7.938073] Btrfs loaded
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    7.972198] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    7.973358] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    8.042647] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    8.048944] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    8.049937] EXT4-fs (sda1): recovery complete
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    8.054398] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    8.247224] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    8.352584] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    8.402362] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    8.581816] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    9.120588] random: cloud-init: uninitialized urandom read (32 bytes read, 47 bits of entropy available)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    9.237086] systemd-udevd[702]: starting version 204
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    9.343873] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    9.381384] intel_rapl: no valid rapl domains found in package 0
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    9.427717] ppdev: user-space parallel port driver
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    9.534131] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    9.579100] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    9.635082] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [    9.796539] random: cloud-init: uninitialized urandom read (32 bytes read, 60 bits of entropy available)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   10.045418] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   10.114406] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   10.184901] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   10.224596] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   10.638124] init: failsafe main process (1094) killed by TERM signal
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 instance-setup: INFO Running set_multiqueue.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 instance-setup: INFO Set channels for eth0 to 4.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 19:55:16 travis-job-cc8b25 device virtio1.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 19:55:16 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   11.300646] random: nonblocking pool is initialized
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Creating a new user account for me.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Created user account me.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Creating a new user account for henrik.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Created user account henrik.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Creating a new user account for emma.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Created user account emma.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Creating a new user account for igor.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Created user account igor.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 cron[1423]: (CRON) INFO (pidfile fd = 3)
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 cron[1467]: (CRON) STARTUP (fork ok)
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 cron[1467]: (CRON) INFO (Running @reboot jobs)
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 google-accounts: INFO Created user account konstantinhaase.
Aug rnel: [   12.112224] Initializing XFRM netlink socket
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   12.118718] Netfilter messages via NETLINK v0.30.
Aug 14 19:55:17 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   12.122091] ctnetlink v0.93: registering with nfnetlink.
Aug 14 19:55:18 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   12.384306] floppy0: no floppy controllers found
Aug 14 19:55:40 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpdate[1839]: adjust time server 169.254.169.254 offset 0.002779 sec
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1864]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1865]: proto: precision = 0.144 usec
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1865]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1865]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1865]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1865]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1865]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1865]: Listen normally on 3 eth0 10.20.0.136 UDP 123
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1865]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1865]: peers refreshed
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ntpd[1865]: Listening on routing socket on fd #21 for interface updates
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   42.060915] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 startup-script: INFO Found startup-script in metadata.
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 startup-script: INFO startup-script: job 1 at Tue Aug 14 23:05:00 2018
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 startup-script: INFO startup-script: Return code 0.
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 startup-script: INFO startup-script: Return code 0.
Aug 14 19:55:47 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 startup-script: INFO Finished running startup scripts.
Aug 14 19:55:48 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ec2: 
Aug 14 19:55:48 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ec2: #############################################################
Aug 14 19:55:48 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 19:55:48 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ec2: 1024 de:ec:1b:4b:72:11:5e:fc:3b:70:e6:68:a6:4c:60:de  root@travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 (DSA)
Aug 14 19:55:48 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ec2: 256 43:1e:d8:1e:9b:29:2b:16:ac:56:47:66:55:c0:c7:05  root@travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 (ECDSA)
Aug 14 19:55:48 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ec2: 256 a4:0f:4f:a3:b3:9b:f5:4d:ff:8c:9a:ca:3e:cc:13:df  root@travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 (ED25519)
Aug 14 19:55:48 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ec2: 2048 01:a9:7e:95:db:37:34:8c:24:2d:d9:12:47:0c:d8:e8  root@travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 (RSA)
Aug 14 19:55:48 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 19:55:48 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 ec2: #############################################################
Aug 14 19:56:18 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [   72.877925] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 19:57:07 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [  121.952880] device vethe8d0762 entered promiscuous mode
Aug 14 19:57:07 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [  121.952972] docker0: port 1(vethe8d0762) entered forwarding state
Aug 14 19:57:07 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [  121.952980] docker0: port 1(vethe8d0762) entered forwarding state
Aug 14 19:57:07 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [  121.953382] docker0: port 1(vethe8d0762) entered disabled state
Aug 14 19:57:07 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [  122.036784] cgroup: docker-runc (4846) created nested.196294] docker0: port 1(vethe8d0762) entered forwarding state
Aug 14 20:17:01 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 CRON[13522]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 14 20:49:06 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [ 3240.390990] veth54be0eb: renamed from eth0
Aug 14 20:49:06 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [ 3240.411026] docker0: port 1(vethe8d0762) entered disabled state
Aug 14 20:49:06 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [ 3240.460744] docker0: port 1(vethe8d0762) entered disabled state
Aug 14 20:49:06 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [ 3240.462455] device vethe8d0762 left promiscuous mode
Aug 14 20:49:06 travis-job-cc8b2505-5e55-4315-8bfa-6d8d47039b47 kernel: [ 3240.462458] docker0: port 1(vethe8d0762) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1676aeef
---
travis_time:end:33e7344d:start=1534279747833265539,finish=1534279747839038397,duration=5772858
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03377c43
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0258657c
travis_time:start:0258657c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:11a0c82e
$ dmesg | grep -i kill
