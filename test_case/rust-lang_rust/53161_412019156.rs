plain
[00:20:49]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
nown-linux-gnu/release/build/rustc_llvm-71c6a17c4b962f9c/out -L native=/usr/lib/llvm-5.0/lib` (exit code: 1)
[00:20:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json"
[00:20:58] expected success, got: exit code: 101
[00:20:58] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:20:58] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:20:58] travis_time:end:stage0-rustc_codegen_llvm:start=1533890812715307602,finish=1533890831139600894,duration=18424293292

[00:20:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:58] Build completed unsuccessfully in 0:15:58
[00:20:58] Makefile:28: recipe for target 'all' failed
[00:20:58] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e301e49
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Aug 10 08:47:11 UTC 2018
Fri, 10 Aug 2018 08:47:11 GMT
travis_time:end:0e301e49:start=1533890831407303960,finish=1533890831461350833,duration=54046873

The command "date && (curl -fs --h 0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0   0.873616] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.876089] PCI: Using configuration type 1 for base access
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.889873] ACPI: Added _OSI(Module Device)
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.891875] ACPI: Added _OSI(Processor Device)
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.893768] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.895453] ACPI: Added _OSI(Processor Aggregator Device)
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.900001] ACPI: Executed 2 blocks of module-level executable AML code
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.924634] ACPI: Interpreter enabled
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.926375] ACPI: (supports S0 S3 S4 S5)
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.928102] ACPI: Using IOAPIC for interrupt routing
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.930745] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.964405] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.967760] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.970684] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.974005] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.979312] PCI host bridge to bus 0000:00
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.980970] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.983678] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.987369] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.990602] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.993433] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.995823] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    0.996261] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    1.017677] pci 0000:00:01.3: [8086:7sion 4.0 (2009/01/31) Phillip Lougher
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.313662] fuse init (API version 7.23)
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.314418] Key type big_key registered
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.315003] Allocating IMA MOK and blacklist keyrings.
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.316808] Key type asymmetric registered
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.317402] Asymmetric key parser 'x509' registered
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.318357] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.319741] io scheduler noop registered
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.320605] io scheduler deadline registered (default)
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.321472] io scheduler cfq registered
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.322130] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.323103] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.324203] intel_idle: does not run on family 6 model 63
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f 
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.659736] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.662880]  sda: sda1
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.663939] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    3.688646] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    4.300437] tsc: Refined TSC clocksource calibration: 2300.000 MHz
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    4.301401] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735223b2, max_idle_ns: 440795277976 ns
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    4.521450] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    6.604584] floppy0: no floppy controllers found
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    7.772408] raid6: sse2x1   gen()  8825 MB/s
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    7.840441] raid6: sse2x1   xor()  6208 MB/s
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    7.908405] raid6: sse2x2   gen() 10778 MB/s
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    7.976397] raid6: sse2x2   xor()  7084 MB/s
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.044463] raid6: sse2x4   gen() 12202 MB/s
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.112397] raid6: sse2x4   xor()  8769 MB/s
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.180414] raid6: avx2x1   gen() 16437 MB/s
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.248422] raid6: avx2x2   gen() 19856 MB/s
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.316397] raid6: avx2x4   gen() 21484 MB/s
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.318313] raid6: using algorithm avx2x4 gen() 21484 MB/s
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.320529] raid6: using avx2x2 recovery algorithm
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.323980] xor: automatically using best checksumming function:
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.364395]    avx       : 26872.000 MB/sec
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.381574] Btrfs loaded
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.438779] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.442868] EXT4-fs (sda1): write access will be enabled during recovery
Aug 10 08:25:16 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [    8.540277] EXT447:11 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [ 1326.354314] device veth3104a3d left promiscuous mode
Aug 10 08:47:11 travis-job-54613ced-4b2f-4e94-8349-f1c00edcd49f kernel: [ 1326.354316] docker0: port 1(veth3104a3d) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:07ad1419
