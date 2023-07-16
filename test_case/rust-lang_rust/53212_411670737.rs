plain
[00:45:06] ....................................................................................................
[00:45:08] ....................................................................................................
[00:45:10] ....................................................................................................
[00:45:14] ....................................................................................................
[00:45:16] ..............iiiiiiiii.............................................................................
[00:45:22] ....................................................................................................
[00:45:25] ....................i...............................................................................
[00:45:28] ..............................i.....................................................................
[00:45:31] ....................................................................................................
---
[00:48:31] ....................................................................................................
[00:48:44] ....................................................................................................
[00:48:54] ....................................................................................................
[00:49:03] .................................................................................i..................
[00:49:14] ....................................................................F............i..................
[00:49:44] ....................................................................................................
[00:49:57] ...................................................ii...............................................
[00:50:13] .............i....i.....................................................i...........................
[00:50:37] ....................................................................................................
[00:50:37] ....................................................................................................
[00:50:47] ....................................................................................................
[00:50:55] ....................................................................................................
[00:51:05] ....................................................................................................
own-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:12] 
[00:51:12] 
[00:51:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:12] Build completed unsuccessfully in 0:08:43
[00:51:12] Build completed unsuccessfully in 0:08:43
[00:51:12] Makefile:58: recipe for target 'check' failed
[00:51:12] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0388953c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Aug  9 07:54:59 UTC 2018
.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] kvm-clock: using sched offset of 2016737651 cycles
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Zone ranges:
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   Device   empty
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Movable zone start for each node
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Early memory node ranges
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 b8
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Policy zone: Normal
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] console [ttyS0] enabled
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.344681] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.346518] pid_max: default: 32768 minimum: 301
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.347349] ACPI: Core revision 20150930
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.354573] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.356017] Security Framework initialized
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.356859] Yama: becoming mindful.
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.357462] AppArmor: AppArmor disabled by boot time parameter
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.359934] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 07:01:54 t62e8c7c38159 kernel: [    0.569492] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.572883] x86: Booting SMP configuration:
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.573953] .... node  #0, CPUs:      #1
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.575062] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.579029]  #2
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.579520] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.583623]  #3
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.584076] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.587357] x86: Booted up 1 node, 4 CPUs
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.587979] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.590326] devtmpfs: initialized
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.595103] evm: security.selinux
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.595689] evm: security.SMACK64
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.596425] evm: security.SMACK64EXEC
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.597026] evm: security.SMACK64TRANSMUTE
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.597689] evm: security.SMACK64MMAP
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.598259] evm: security.ima
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.598724] evm: security.capability
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.599630] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.601075] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.602165] pinctrl core: initialized pinctrl subsystem
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.603067] RTC time:  7:01:44, date: 08/09/18
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.604644] NET: Registered protocol family 16
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.615885] cpuidle: using governor ladder
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.627872] cpuidle: using governor menu
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.628667] PCCT header not found.
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.629516] ACPI: bus type PCI registered
Aug 093-ae17-62e8c7c38159 kernel: [    0.738448] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.753979] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.755551] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.762207] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.766723] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.781977] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.787984] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.793232] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.809310] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.812073] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.814720] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.817171] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.819763] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.841527] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.843220] vgaarb: loaded
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.843858] SCSI subsystem initialized
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.844719] libata version 3.00 loaded.
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.844756] ACPI: bus type USB registered
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.845419] usbcore: registered new interface driver usbfs
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.846297] usbcore: registered new interface driver hub
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.847137] usbcore: registered new device driver usb
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.848132] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.849212] dmi: Firmware registration failed.
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.850170] PCI: Using ACPI for IRQ routing
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.850758] PCI: pci_cache_line_size set to 64 bytes
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.850860] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.850862] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.850997] NetLabel: Initializing
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.851525] NetLabel:  domain hash size = 128
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.852274] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.853023] NetLabel:  unlabeled traffic allowed by default
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.853940] amd_nb: Cannot enumerate AMD northbridges
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.854724] clocksource: Switched to clocksource kvm-clock
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.863710] pnp: PnP ACPI init
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.864311] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.864382] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.864427] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    0.864483] pnp 00:les, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.013692] hw unit of domain pp0-core 2^-0 Joules
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.014452] hw unit of domain package 2^-0 Joules
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.015531] hw unit of domain dram 2^-0 Joules
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.016320] Scanning for low memory corruption every 60 seconds
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.017814] audit: initializing netlink subsys (disabled)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.018659] audit: type=2000 audit(1533798107.084:1): initialized
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.020114] Initialise system trusted keyring
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.021017] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.022135] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.024590] zbud: loaded
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.025335] VFS: Disk quotas dquot_6.6.0
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.025961] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 07:01:54 travis-job-a78b 45
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.039935] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.041411] ACPI: Power Button [PWRF]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.042186] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.043409] ACPI: Sleep Button [SLPF]
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.044726] GHES: HEST is not enabled!
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.047155] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.048212] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.054414] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.055419] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.060465] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.083284] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.106685] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.130691] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.154972] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.159090] Linux agpgart interface v0.103
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.162501] loop: module loaded
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.163593] libphy: Fixed MDIO Bus: probed
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.164615] tun: Universal TUN/TAP device driver, 1.6
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.165938] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.205134] PPP generic driver version 2.4.2
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.206781] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.208711] ehci-pci: EHCI PCI platform driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.210006] ehci-platform: EHCI generic platform driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.211348] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.212940] ohci-pci: OHCI PCI platform driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.214114] ohci-platform: OHCI generic platform driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.215731] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.217607] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.220993] i8042: Warning: Keylock active
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.223091] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.224253] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.226522] mousedev: PS/2 mouse device common for all mice
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.228506] rtc_cmos 00:00: RTC can wake from S4
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.230239] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.232208] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.234261] i2c /dev entries driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ke01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.279018] Freeing unused kernel memory: 1496K
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.280057] Write protecting the kernel read-only data: 14336k
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.281830] Freeing unused kernel memory: 1956K
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.283297] Freeing unused kernel memory: 92K
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.299387] systemd-udevd[118]: starting version 204
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.357626] scsi host0: Virtio SCSI HBA
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.365129] AVX version of gcm_enc/dec engaged.
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.366039] AES CTR mode by8 optimization enabled
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.366060] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.401585] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.401647] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.404482] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.406107] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.407267] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.407414] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.411353]  sda: sda1
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.413305] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    3.431105] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    4.014904] tsc: Refined TSC clocksource calibration: 2599.767 MHz
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    4.016010] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x25796011374, max_idle_ns: 440795332112 ns
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    4.267907] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    6.342888] floppy0: no floppy controllers found
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    7.502737] raid6: sse2x1   gen()  9032 MB/s
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    7.570740] raid6: sse2x1   xor()  7032 MB/s
Aug  9 07:01:54 travis-jintel_rapl: no valid rapl domains found in package 0
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    9.439616] ppdev: user-space parallel port driver
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    9.518512] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    9.565716] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    9.632096] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [    9.792694] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   10.063926] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   10.141417] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   10.213863] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   10.251394] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   10.608178] init: failsafe main process (1095) killed by TERM signal
Aug  9 07:01:54 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Running set_multiqueue.
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Set channels for eth0 to 4.
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  9 07:01:55 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  9 07:01:555658] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 07:01:56 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   11.998789] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 07:01:56 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   12.001602] Bridge firewalling registered
Aug  9 07:01:56 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   12.011653] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 07:01:56 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   12.078968] Initializing XFRM netlink socket
Aug  9 07:01:56 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   12.087573] Netfilter messages via NETLINK v0.30.
Aug  9 07:01:56 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   12.090911] ctnetlink v0.93: registering with nfnetlink.
Aug  9 07:01:56 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   12.422906] floppy0: no floppy controllers found
Aug  9 07:02:19 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpdate[1778]: adjust time server 169.254.169.254 offset 0.006192 sec
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1813]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: proto: precision = 0.100 usec
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: Listen normally on 3 eth0 10.20.0.86 UDP 123
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: peers refreshed
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: Listening on routing socket on fd #21 for interface updates
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [   42.088902] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 startup-script: INFO Found startup-script in metadata.
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 startup-script: INFO startup-script: job 1 at Thu Aug  9 10:12:00 2018
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 startup-script: INFO startup-script: job 1 at Thu Aug  9 10:12:00 2018
Aug  9 07:02:26 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 startup-script: INFO startup-script: Returnon 5 docker0 fe80::1 UDP 123
Aug  9 07:04:51 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: Listen normally on 6 docker0 fe80::42:5bff:fe86:7e0b UDP 123
Aug  9 07:04:51 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  9 07:04:51 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: peers refreshed
Aug  9 07:04:51 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 ntpd[1814]: new interface(s) found: waking up resolver
Aug  9 07:05:03 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [  199.157505] docker0: port 1(vethdf5f4e5) entered forwarding state
Aug  9 07:17:01 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 CRON[12591]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  9 07:49:28 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 2864.010129] traps: a[5268] trap invalid opcode ip:5651322feb1b sp:7fffbf241660 error:0 in a[5651322fb000+6000]
Aug  9 07:49:42 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 2878.137723] traps: a[8092] trap invalid opcode ip:7fd99788dea1 sp:7ffc386a35d0 error:0 in libstd-2339b911e3c09de8.so[7fd997833000+172000]
Aug  9 07:49:42 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 2878.189977] traps: a[8093] trap invalid opcode ip:7f266ea26ea1 sp:7ffc534ec3f0 error:0 in libstd-2339b911e3c09de8.so[7f266e9cc000+172000]
Aug  9 07:51:01 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 2956.959790] traps: a[23021] trap invalid opcode ip:55b0365f5d68 sp:7ffca99f6a10 error:0 in a[55b0365f3000+4000]
Aug  9 07:53:39 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 3114.397654] a[19207]: segfault at 0 ip 0000560f4b315548 sp 00007fff02de36d0 error 6 in a[560f4b312000+5000]
Aug  9 07:53:47 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 3122.926640] a[19973]: segfault at 1 ip 0000561869a11b5c sp 00007ffc1bf8d130 error 6 in a[561869a0f000+4000]
Aug  9 07:53:51 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 3126.633466] traps: a[20335] trap invalid opcode ip:55e2d01da42c sp:7ffc83d654d0 error:0 in a[55e2d01d7000+7000]
Aug  9 07:54:59 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 3194.773377] veth689b143: renamed from eth0
Aug  9 07:54:59 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 3194.785648] docker0: port 1(vethdf5f4e5) entered disabled state
Aug  9 07:54:59 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 3194.834386] docker0: port 1(vethdf5f4e5) entered disabled state
Aug  9 07:54:59 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 3194.836033] device vethdf5f4e5 left promiscuous mode
Aug  9 07:54:59 travis-job-a78b1daa-f67f-4093-ae17-62e8c7c38159 kernel: [ 3194.836035] docker0: port 1(vethdf5f4e5) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:018c2e44
---
travis_time:end:00a450f5:start=1533801300981766949,finish=1533801301063631881,duration=81864932
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b81fc04
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:1a953bc9
$ dmesg | grep -i kill
