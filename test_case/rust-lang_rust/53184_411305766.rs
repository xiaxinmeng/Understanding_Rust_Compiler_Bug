plain
[00:46:05] ....................................................................................................
[00:46:08] ....................................................................................................
[00:46:11] ....................................................................................................
[00:46:14] ....................................................................................................
[00:46:18] .................i........................................................................F.........
[00:46:24] ....................................................................................................
[00:46:28] ....................................................................................................
[00:46:31] ..................................................i.................................................
[00:46:33] ..........................................i....
---
[00:46:33] -   --> $DIR/issue-52742.rs:25:9
[00:46:33] + error[E0106]: missing lifetime specifiers
[00:46:33] +   --> $DIR/issue-52742.rs:23:10
[00:46:33] 3    |
[00:46:33] - LL |     fn take_bar(&mut self, b: Bar<'_>) {
[00:46:33] -    |                 ---------         -- let's call this `'1`
[00:46:33] -    |                 |
[00:46:33] -    |                 has type `&mut Foo<'_, '2>`
[00:46:33] - LL |         self.y = b.z
[00:46:33] -    |         ^^^^^^^^^^^^ requires that `'1` must outlive `'2`
[00:46:33] + LL | impl Foo<'_, '_> {
[00:46:33] +    |          ^^ expected 2 lifetime parameters
[00:46:33] 11 error: aborting due to previous error
[00:46:33] 12 
[00:46:33] 
[00:46:33] + For more information about this error, try `rustc --explain E0106`.
[00:46:33] + For more information about this error, try `rustc --explain E0106`.
[00:46:33] 13 
[00:46:33] 
[00:46:33] 
[00:46:33] The actual stderr differed from the e/github.com/rust-lang/rust/issues/15872\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-52742.rs","byte_start":616,"byte_end":618,"line_start":23,"line_end":23,"column_start":10,"column_end":12,"is_primary":true,"text":[{"text":"impl Foo<'_, '_> {","highlight_start":10,"highlight_end":12}],"label":"expected 2 lifetime parameters","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0106]: missing lifetime specifiers\n  --> /checkout/src/test/ui/nll/issue-52742.rs:23:10\n   |\nLL | impl Foo<'_, '_> {\n   |          ^^ expected 2 lifetime parameters\n\n"}
[00:46:33] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:33] {"message":"For more information about this error, try `rustc --explain E0106`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0106`.\n"}
[00:46:33] ------------------------------------------
[00:46:33] 
[00:46:33] thread '[ui] ui/nll/issue-52742.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:46:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:33] 
[00:46:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:46:33] 
[00:46:33] 
[00:46:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:33] 
[00:46:33] 
[00:46:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:33] Build completed unsuccessfully in 0:02:09
[00:46:33] Build completed unsuccessfully in 0:02:09
[00:46:33] make: *** [check] Error 1
[00:46:33] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c3fe9ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:175b5628
$ sudo tail -n 500 /var/log/syslog
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kern5728 pages, LIFO batch:31
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 06:06:51 travis-job-bs-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] console [ttyS0] enabled
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.317333] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.318710] pid_max: default: 32768 6:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.553584] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.557939] x86: Booted up 1 node, 4 CPUs
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.559042] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.561495] devtmpfs: initialized
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.566052] evm: security.selinux
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.566675] evm: security.SMACK64
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.567238] evm: security.SMACK64EXEC
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.567753] evm: security.SMACK64TRANSMUTE
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.568422] evm: security.SMACK64MMAP
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.569011] evm: security.ima
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.569492] evm: security.capability
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.570398] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.572045] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.573456] pinctrl core: initialized pinctrl subsystem
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.574519] RTC time:  6:06:40, date: 08/08/18
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.576126] NET: Registered protocol family 16
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.588161] cpuidle: using governor ladder
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.600147] cpuidle: using governor menu
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.600864] PCCT header not found.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.601599] ACPI: bus type PCI registered
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.602480] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.603607] PCI: Using configuration type 1 for base access
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.617181] ACPI: Added _OSI(Module Device)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.617920] ACPI: Added _OSI(Processor Device)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.618573] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.619481] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1vis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.755600] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.760887] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.775960] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.778541] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.781017] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.783572] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.785804] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.806866] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.808215] vgaarb: loaded
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.808802] SCSI subsystem initialized
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.809632] libata version 3.00 loaded.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.809660] ACPI: bus type USB registered
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.810292] usbcore: registered new interface driver usbfs
Auga3-fa1351e30c67 kernel: [    0.837982] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.837985] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.837986] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.837988] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.838023] NET: Registered protocol family 2
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.838982] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.840289] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.841434] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.842527] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.843514] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.845300] NET: Registered protocol family 1
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    0.845956] pci 0000:00:00.0: Limiting direct PCI/P  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.126994] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.165701] PPP generic driver version 2.4.2
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.167055] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.168956] ehci-pci: EHCI PCI platform driver
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.170419] ehci-platform: EHCI generic platform driver
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.171787] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.173221] ohci-pci: OHCI PCI platform driver
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.174411] ohci-platform: OHCI generic platform driver
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.175854] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.177591] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.180731] i8042: Warning: Keylock active
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.182920] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 06:06:51 travis-job-bdc345af24-aca3-fa1351e30c67 kernel: [    3.318746] AVX version of gcm_enc/dec engaged.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.319449] AES CTR mode by8 optimization enabled
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.353404] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.353410] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.355527] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.356644] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.357410] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.357496] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.360117]  sda: sda1
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.361364] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.389011] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.976196] tsc: Refined TSC clocksource calibration: 2499.782 MHz
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    3.977354] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086be2dcf, max_idle_ns: 440795324453 ns
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    4.226085] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    6.296287] floppy0: no floppy controllers found
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    7.464205] raid6: sse2x1   gen()  9256 MB/s
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    7.532127] raid6: sse2x1   xor()  6957 MB/s
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    7.600112] raid6: sse2x2   gen() 11500 MB/s
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    7.668116] raid6: sse2x2   xor()  7709 MB/s
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    7.736113] raid6: sse2x4   gen() 12663 MB/s
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    7.804118] raid6: sse2x4   xor()  8656 MB/s
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    7.805203] raid6: using algorithm sse2x4 gen() 12663 MB/s
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    7.806224] raid6: .... xor() 8656 MB/s, rmw enabled
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    7.807249] raid6: using ssse3x2 recovery algorithm
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    7.809392] xor: automatikernel: [    8.502543] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    9.024901] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    9.150708] systemd-udevd[699]: starting version 204
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    9.247887] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    9.340846] ppdev: user-space parallel port driver
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    9.444683] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    9.493156] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    9.561572] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [    9.721809] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [   10.196496] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  8 06:06:51 travi-0c7a-4e24-aca3-fa1351e30c67 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [   11.434066] random: nonblocking pool is initialized
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 google-accounts: INFO Starting Google Accounts daemon.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 google-accounts: INFO Creating a new user account for me.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 google-accounts: INFO Created user account me.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 google-accounts: INFO Creating a new user account for aj.
Aug  8 06:06:51 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 google-accounts: INFO Created user account aj.
Aug  8 06:06:52 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 06:06:t: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 startup-script: INFO startup-script: Return code 0.
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 startup-script: INFO Finished running startup scripts.
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ec2: 
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ec2: 
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ec2: #############################################################
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ec2: 1024 94:00:6b:b7:6b:fd:09:82:63:6f:65:76:78:7b:60:34  root@travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 (DSA)
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ec2: 256 20:58:7f:65:af:6a:fc:2e:37:89:1b:05:2c:2a:80:95  root@travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 (ECDSA)
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ec2: 256 15:a0:87:5e:f8:de:0e:90:58:32:95:96:05:d0:fa:61  root@travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 (ED25519)
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ec2: 2048 d1:fb:b5:95:33:c4:3b:c6:e1:64:e0:9b:2a:5e:23:7c  root@travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 (RSA)
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 06:07:22 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ec2: #############################################################
Aug  8 06:08:42 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [  122.411641] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 06:09:45 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [  185.705622] device veth0a54224 entered promiscuous mode
Aug  8 06:09:45 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [  185.793496] cgroup: docker-runc (4783) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 06:09:45 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [  185.793498] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 06:09:46 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [  185.859989] eth0: renamed from vethfe61536
Aug  8 06:09:46 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [  185.902865] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 06:09:46 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [  185.904013] docker0: port 1(veth0a54224) entered forwarding state
Aug  8 06:09:46 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [  185.904030] docker0: port 1(veth0a54224) entered forwarding state
Aug  8 06:09:46 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [  185.904054] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 06:09:49 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ntpd[1793]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  8 06:09:49 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ntpd[1793]: L7a-4e24-aca3-fa1351e30c67 ntpd[27703]: proto: precision = 0.105 usec
Aug  8 06:31:31 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ntpd[27703]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 06:31:31 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ntpd[27703]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 06:31:31 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ntpd[27703]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 06:31:31 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ntpd[27703]: Listen normally on 3 eth0 10.20.0.4 UDP 123
Aug  8 06:31:31 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ntpd[27703]: Listen normally on 3 eth0 10.20.0.4 UDP 123
Aug  8 06:31:31 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ntpd[27703]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 06:31:31 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ntpd[27703]: peers refreshed
Aug  8 06:31:31 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 ntpd[27703]: Listening on routing socket on fd #21 for interface updates
Aug  8 06:31:33 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 06:31:33 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 06:31:36 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 dbus[1148]: [system] Reloaded configuration
Aug  8 06:55:17 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [ 2917.084438] vethfe61536: renamed from eth0
Aug  8 06:55:17 travis-job-bdc345af-0c7a-4e24-aca3-fa1351e30c67 kernel: [ 2917.093660932 .
2298132 ./obj/build
1694520 ./obj/build/x86_64-unknown-linux-gnu
792736 ./src
569516 ./.git
---
154344 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
149112 ./src/llvm-emscripten/test
147700 ./obj/build/bootstrap/debug/incremental
133276 ./obj/build/bootstrap/debug/incremental/bootstrap-jdsuci5s9dha
133272 ./obj/build/bootstrap/debug/incremental/bootstrap-jdsuci5s9dha/s-f3nk0xpowd-17n5kin-cu8hwe6rzsbr
128636 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
125324 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
125320 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
122576 ./obj/build/x86_64-unknoe.6
