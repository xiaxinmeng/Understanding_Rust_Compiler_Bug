\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/index-mut-help.rs","byte_start":832,"byte_end":844,"line_start":23,"line_end":23,"column_start":18,"column_end":30,"is_primary":true,"text":[{"text":"    let _ = &mut map[\"peter\"];      //~ ERROR","highlight_start":18,"highlight_end":30}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"trait `IndexMut` is required to modify indexed content, but it is not implemented for `std::collections::HashMap<&str, std::string::String>`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable indexed content as mutable\n  --> /checkout/src/test/ui/borrowck/index-mut-help.rs:23:18\n   |\nLL |     let _ = &mut map[\"peter\"];      //~ ERROR\n   |                  ^^^^^^^^^^^^ cannot borrow as mutable\n   |\n   = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `std::collections::HashMap<&str, std::string::String>`\n\n"}
[00:44:40] {"message":"aborting due to 3 previous errors","code":null,"le:612,"byte_end":632,"line_start":15,"line_end":15,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"        things[src.as_str()].sort(); //~ ERROR cannot borrow immutable","highlight_start":9,"highlight_end":29}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"trait `IndexMut` is required to modify indexed content, but it is not implemented for `std::collections::HashMap<std::string::String, std::vec::Vec<std::string::String>>`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable indexed content as mutable\n  --> /checkout/src/test/ui/issue-41726.rs:15:9\n   |\nLL |         things[src.as_str()].sort(); //~ ERROR cannot borrow immutable\n   |         ^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable\n   |\n   = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `std::collections::HashMap<std::string::String, std::vec::Vec<std::string::String>>`\n\n"}
[00:44:40] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:40] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:44:40] ------------------------------------------
[00:44:40] 
[00:44:40] thread '[ui] ui/issue-41726.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:44:40] 
---
[00:44:40] 
[00:44:40] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:44:40] 
[00:44:40] 
[00:44:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:40] expected success, got: exit code: 10PI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] SRAT: PXM 0 -> idle_ns: 881590591483 ns
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] Zone ranges:
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000]   Device   empty
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] Movable zone start for each node
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] Early memory node ranges
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c3s-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] Policy zone: Normal
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.000000] R47d4-875e-f9c302c67dc4 kernel: [    0.375555] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.380646] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.382521] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.384462] Initializing cgroup subsys io
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.385108] Initializing cgroup subsys memory
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.385975] Initializing cgroup subsys devices
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.387087] Initializing cgroup subsys freezer
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.387817] Initializing cgroup subsys net_cls
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.388597] Initializing cgroup subsys perf_event
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.389351] Initializing cgroup subsys net_prio
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.390102] Initializing cgroup subsys hugetlb
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.391092] Initializing cgroup subsys pids
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.392167] CPU: Physical Processor ID: 0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.393210] CPU: Processor Core ID: 0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.393765] mce: CPU supports 32 MCE banks
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.394863] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.395945] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.398859] Freeing SMP alternatives memory: 32K
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.408127] ftrace: allocating 32185 entries in 126 pages
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.456711] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.457769] smpboot: Max logical packages: 2
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.459144] x2apic enabled
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.460841] Switched APIC routing to physical x2apic.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.464544] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.570353] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.572008] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.575524] x86: Booting SMP configuration:
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.576485] .... node  #0, CPUs:      #1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.577370] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.580836]  #2
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.581298] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.585620]  #3
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.586165] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.589354] x86: Booted up 1 node, 4 CPUs
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.590294] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.592728] devtmpfs: initialized
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.597925] evm: security.selinux
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.598452] evm: security.SMACK64
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.598986] evm: security.SMACK64EXEel: [    0.711668] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.712799] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.713841] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.716577] PCI host bridge to bus 0000:00
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.717345] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.718512] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.720015] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.721137] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.722187] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.723158] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.723611] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kef81-47d4-875e-f9c302c67dc4 kernel: [    0.829570] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.850657] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.851842] vgaarb: loaded
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.852438] SCSI subsystem initialized
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.853150] libata version 3.00 loaded.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.853201] ACPI: bus type USB registered
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.853984] usbcore: registered new interface driver usbfs
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.854908] usbcore: registered new interface driver hub
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.855936] usbcore: registered new device driver usb
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.857357] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.858838] dmi: Firmware registration failed.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.860013] PCI: Using ACPI for IRQ routing
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    0.860684] PCI: pci_cache_line_size set to 64 bytes
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c737418240 ms ovfl timer
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.018148] hw unit of domain pp0-core 2^-0 Joules
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.019490] hw unit of domain package 2^-0 Joules
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.020620] hw unit of domain dram 2^-0 Joules
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.021838] Scanning for low memory corruption every 60 seconds
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.023736] audit: initializing netlink subsys (disabled)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.024923] audit: type=2000 audit(1533676193.676:1): initialized
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.026690] Initialise system trusted keyring
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.027733] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.029021] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.031482] zbud: loaded
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.032289] VFS: Disk quotas dquot_6.6.0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.033370] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.034699] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.036333] fuse init (API version 7.23)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.037353] Key type big_key registered
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.038316] Allocating IMA MOK and blacklist keyrings.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.040554] Key type asymmetric registered
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.041373] Asymmetric key parser 'x509' registered
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.042381] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.043600] io scheduler noop registered
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.044322] io scheduler deadline registered (default)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.045155] io scheduler cfq registered
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.045968] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.046897] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.048163] intel_idle: does not run on family 6 model 45
Aug  7 21:10:02 tra15200) is a 16550A
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.140499] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.164383] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.168031] Linux agpgart interface v0.103
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.171058] loop: module loaded
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.172193] libphy: Fixed MDIO Bus: probed
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.173295] tun: Universal TUN/TAP device driver, 1.6
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.174178] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.216182] PPP generic driver version 2.4.2
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.217533] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.219249] ehci-pci: EHCI PCI platform driver
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.220472] ehci-platform: EHCI generic platform driver
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.221611] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.222748] ohci-pci: OHCI PCI platform driver
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.223902] ohci-platform: OHCI generic platform driver
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.225009] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.226411] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.228827] i8042: Warning: Keylock active
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.230659] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.231650] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.233019] mousedev: PS/2 mouse device common for all mice
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.234743] rtc_cmos 00:00: RTC can wake from S4
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.236171] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.237661] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.239412] i2c /dev entries driver
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.240215] device-mapper: uevent: version 1.0.3
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.241394] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.243349] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.245062] NET: Registered protocol family 10
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.246178] NET: Registered protocol family 17
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.247102] Key type dns_resolver registered
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.248274] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.249983] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.251110] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.252259] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.253696] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.256056] registered taskstats version 1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.256836] Loading co5] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.413172] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.416176]  sda: sda1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.417463] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    3.437828] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    4.021610] tsc: Refined TSC clocksource calibration: 2599.783 MHz
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    4.022724] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x25796f27178, max_idle_ns: 440795276799 ns
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    4.270619] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    6.345670] floppy0: no floppy controllers found
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    7.513491] raid6: sse2x1   gen()  9168 MB/s
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    7.581497] raid6: sse2x1   xor()  6910 MB/s
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    7.649492] raid6: sse2x2   gen() 11405 MB/s
Aug  7 21:10:02 travis-job-f12835c5-3f81-47davis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    8.036398] EXT4-fs (sda1): recovery complete
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    8.041910] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    8.264811] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    8.379831] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    8.436076] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    8.645079] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    9.223332] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    9.358994] systemd-udevd[702]: starting version 204
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    9.470331] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    9.527242] intel_rapl: no valid rapl domains found in package 0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [    9.586598] console': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO Running set_multiqueue.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 21:10:02 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 insta19:31:29.715998981 +0000]
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 cron[1435]: (CRON) INFO (pidfile fd = 3)
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 cron[1470]: (CRON) STARTUP (fork ok)
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 cron[1470]: (CRON) INFO (Running @reboot jobs)
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 acpid: starting up with netlink and the input layer
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 acpid: 1 rule loaded
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 acpid: waiting for events: event logging is off
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 haveged: haveged starting up
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [   12.175521] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [   12.192102] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 21:10:03 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [   12.233926] bcard 0.0.0.0 UDP 123
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: Listen normally on 3 eth0 10.20.255.4 UDP 123
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: peers refreshed
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: peers refreshed
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: Listening on routing socket on fd #21 for interface updates
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [   42.398282] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 startup-script: INFO Found startup-script in metadata.
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 startup-script: INFO startup-script: job 1 at Wed Aug  8 00:20:00 2018
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 startup-script: INFO startup-script: Return code 0.
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 startup-script: INFO startup-script: Return code 0.
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 startup-script: INFO Finished running startup scripts.
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ec2: 
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ec2: #############################################################
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ec2: 1024 1a:87:25:07:c4:5d:df:80:99:f8:9b:f5:c1:d4:81:e7  root@travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 (DSA)
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ec2: 256 24:78:0e:a2:1f:a0:7f:ea:68:a3:1b:d0:d6:f7:f4:b9  root@travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 (ECDSA)
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ec2: 256 b7:93:1a:42:fd:5e:98:2e:83:2b:51:32:78:46:df:bf  root@travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 (ED25519)
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ec2: 2048 5b:7d:a2:9b:e9:d7:ea:73:74:d4:00:f4:82:4d:05:3a  root@travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 (RSA)
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 21:10:33 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ec2: #############################################################
Aug  7 21:11:53 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [  122.656590] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 21:12:55 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [  184.307754] device veth53d2bd8 entered promiscuous mode
Aug  7 21:12:55 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [  184.401673] cgroup: docker-runc (4770) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 21:12:55 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [  184.401677] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 21:12:55 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [  184.474024] eth0: renamed from vethebfe62f
Aug  7 21:12:55 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [  184.512607] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 21:12:55 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [  184.514024] docker0: port 1(veth53d2bd8) entered forwarding state
Aug  7 21:12:55 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [  184.514041] docker0: port 1(veth53d2bd8) entered forwarding state
Aug  7 21:12:55 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 kernel: [  184.514070] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 21:12:59 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: Listen normally on 5 docker0 fe80::42:ecff:fe52:1298 UDP 123
Aug  7 21:12:59 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  7 21:12:59 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 21:12:59 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: peers refreshed
Aug  7 21:12:59 travis-job-f12835c5-3f81-47d4-875e-f9c302c67dc4 ntpd[1781]: new interface(s) found: waking up resolver
Aug  7 21:13:10 travis-job-f12835c5-3f81-47d4/release/build
34336 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects/pack
34196 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
33884 ./src/llvm-emscripten/lib/Target
32352 ./src/libcompiler_builtins/compiler-rt/test
