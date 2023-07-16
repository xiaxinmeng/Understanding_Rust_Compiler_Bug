plain
[00:25:14]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:25:19]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:26:51]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:27:00]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:28:54] error: the feature `iterator_find_map` has been stable since 1.30 and no longer requires an attribute to enable
[00:28:54]    |
[00:28:54] 48 | #![feature(iterator_find_map)]
[00:28:54]    |            ^^^^^^^^^^^^^^^^^
[00:28:54]    |
[00:28:54]    |
[00:28:54]    = note: `-D stable-features` implied by `-D warnings`
[00:28:54] 
[00:28:55] error: aborting due to previous error
[00:28:55] 
[00:28:55] error: Could not compile `rustc`.
[00:28:55] 
[00:28:55] Caused by:
[00:28:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=3be8fb23eaf1b952 -C extra-filename=-3be8fb23eaf1b952 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-4701371fdd2d83ac.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-8091a0e1fe9f036b.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9b04b6981047a227.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-da03a033b1a119c5.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-30c883ec86c7b095.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-f9eceeb1557f436c.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-e7104d970c23d2cc.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-841a904682cb156a.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-359050b249d8a663.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-8c4b00a8bc5b69e8.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-740445503e73846f.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-c6c0540b399ba5f8.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-18b18a9c466a8028.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-6e9724c8108da5df.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-717d0db29aa6e38e.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-311f65429b65fc66.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-a7669038bd6c823d/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-b967192cda86eebc/out` (exit code: 1)
[00:28:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:28:55] expected success, got: exit code: 101
[00:28:55] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:28:55] travis_fold:end:stage1-rustc

[00:28:55] travis_time:end:stage1-rustc:start=1534333146656660157,finish=1534333423210212631,duration=276553552474


[00:28:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:28:55] Build completed unsuccessfully in 0:23:52
[00:28:55] make: *** [all] Error 1
[00:28:55] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12c2a668
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:08ca7869
$ sudo tail -n 500 /var/log/syslog
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000]   7 disabled
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Using GB pages for direct mapping
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] kvm-clock: using sched offset of 1805935817 cycles
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Zone ranges:
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e--436f-a92e-0d6f4fc03baf kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Policy zone: Normal
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] console [ttyS0] enabled
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.657493] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.662154] pid_max: default: 32768 minimum: 301
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.664611] ACPI: Core revision 20150930
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.672915] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.676893] Security Framework initialized
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.680519] Yama: becoming mindful.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.683333] AppArmor: AppArmor disabled by boot time parameter
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.687974] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.701115] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.709805] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.713843] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.718803] Initializing cgroup subsys io
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kerneMP alternatives memory: 32K
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.778206] ftrace: allocating 32185 entries in 126 pages
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.835736] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.839903] smpboot: Max logical packages: 2
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.844000] x2apic enabled
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.847396] Switched APIC routing to physical x2apic.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.854091] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.961746] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.967456] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.974110] x86: Booting SMP configuration:
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.976983] .... node  #0, CPUs:      #1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.979855] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.987050]  #2
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.988232] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.995326]  #3
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    0.997285] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.004651] x86: Booted up 1 node, 4 CPUs
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.006449] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.012996] devtmpfs: initialized
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.019937] evm: security.selinux
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.023749] evm: security.SMACK64
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.025625] evm: security.SMACK64EXEC
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.028841] evm: security.SMACK64TRANSMUTE
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.032243] evm: security.SMACK64MMAP
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.035088] evm: security.ima
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.037123] evm: security.capability
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.039980] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.046244] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.051299] pinctrl core: initialized pinctrl subsystem
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.055401] RTC time: 11:13:47, date: 08/15/18
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.059568] NET: Registered protocol family 16
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.073877] cpuidle: using governor ladder
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.085854] cpuidle: using governor menu
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.089963] PCCT header not found.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.093213] ACPI: bus type PCI registered
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.096828] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.102123] PCI: Using configuration type 1 for base access
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.120012] ACPI: Added _OSI(Module Device)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.123284] ACPI: Added _OSI(Processor Device)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.125912] ACPI: Added 436f-a92e-0d6f4fc03baf kernel: [    1.463612] ACPI: bus type USB registered
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.466108] usbcore: registered new interface driver usbfs
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.468825] usbcore: registered new interface driver hub
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.471817] usbcore: registered new device driver usb
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.475292] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.479748] dmi: Firmware registration failed.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.482759] PCI: Using ACPI for IRQ routing
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.485505] PCI: pci_cache_line_size set to 64 bytes
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.485649] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.485652] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.485799] NetLabel: Initializing
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.488241] NetLabel:  domain hash size = 128
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.491241] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.494124] NetLabel:  unlabeled traffic allowed by default
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.498015] amd_nb: Cannot enumerate AMD northbridges
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.502360] clocksource: Switched to clocksource kvm-clock
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.513634] pnp: PnP ACPI init
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.515776] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.515853] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.515895] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.515944] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.515984] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.516024] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.516064] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.516227] pnp: PnP ACPI: found 7 devices
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.527865] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.533582] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.533586] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.533587] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.533589] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.533630] NET: Registered protocol family 2
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.536513] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.541947] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.546341] TCP: Hash tables configured (established 131072 bind 65536)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.550662] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    1.554641] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 11:13:59 travis-job-3e023bf4fc03baf kernel: [    3.619456] audit: initializing netlink subsys (disabled)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.620484] audit: type=2000 audit(1534331629.643:1): initialized
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.621733] Initialise system trusted keyring
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.622823] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.624211] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.626997] zbud: loaded
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.627735] VFS: Disk quotas dquot_6.6.0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.628624] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.630585] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.632428] fuse init (API version 7.23)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.633472] Key type big_key registered
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.634131] Allocating IMA MOK and blacklist keyrings.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.636392] Key type asymmetric registered
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.637095] Asymmetric key parser 'x509' registered
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.638176] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.639771] io scheduler noop registered
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.640479] io scheduler deadline registered (default)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.641291] io scheduler cfq registered
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.641941] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.642980] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.644133] intel_idle: does not run on family 6 model 63
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.644236] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.645428] ACPI: Power Button [PWRF]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.646097] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.647213] ACPI: Sleep Button [SLPF]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.648352] GHES: HEST is not enabled!
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.651025] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.652149] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.657330] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.658215] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.663168] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.686027] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.709094] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.732907] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.756885] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.761038] Linux agpgart interface v0.103
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.764778] loop: module loaded
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.766124] libphy: Fixed MDIO Bus: probed
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.767341] tun: Universal TUN/TAP device driver, 1.6
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.768868] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.811333] PPP generic driver version 2.4.2
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.812811] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.815103] ehci-pci: EHCI PCI platform driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.816657] ehci-platform: EHCI generic platform driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.818216] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.820046] ohci-pci: OHCI PCI platform driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.821462] ohci-platform: OHCI generic platform driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.822959] uhci_hcd: USB Universal Host Controller Interface driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.825095] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.827984] i8042: Warning: Keylock active
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.830154] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.831370] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.833443] mousedev: PS/2 mouse device common for all mice
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.835654] rtc_cmos 00:00: RTC can wake from S4
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.837496] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.839845] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.841993] i2c /dev entries driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.843366] device-mapper: uevent: version 1.0.3
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.844836] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.848102] ledtrig-cpu: registered to indicate activity on CPUs
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.850749] NET: Registered protocol family 10
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.852683] NET: Registered protocol family 17
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.854160] Key type dns_resolver registered
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.856059] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.857292] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.858836] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.860949] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.863134] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.866971] registered taskstats version 1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.868275] Loading compiled-in X.509 certificates
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.870774] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.874112] zswap: loaded using pool lzo/zbud
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.878102] Key type trusted registered
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    3.882879] Key type encrypted registered
Aug 15 11:13:59 travis--9766-436f-a92e-0d6f4fc03baf kernel: [    8.622443] raid6: avx2x2   gen() 20291 MB/s
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.690427] raid6: avx2x4   gen() 22420 MB/s
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.693055] raid6: using algorithm avx2x4 gen() 22420 MB/s
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.696861] raid6: using avx2x2 recovery algorithm
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.702505] xor: automatically using best checksumming function:
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.746378]    avx       : 26486.000 MB/sec
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.763118] Btrfs loaded
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.838626] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.842208] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.963579] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.977287] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.981336] EXT4-fs (sda1): recovery complete
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    8.993288] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    9.312889] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    9.509363] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    9.574484] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [    9.878598] random: cloud-init: uninitialized urandom read (32 bytes read, 33 bits of entropy available)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   10.644351] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   10.811340] systemd-udevd[702]: starting version 204
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   10.961500] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   11.031057] intel_rapl: no valid rapl domains found in package 0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   11.083274] intel_rapl: no valid rapl domains found in package 0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   11.091855] ppdev: user-space parallel port driver
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   11.253530] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   11.317917] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   11.388907] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   11.569356] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   11.871814] random: mktemp: uninitialized urandom read (12 bytes read, 55 bits of entropy available)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   11.963224] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   12.059283] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   12.121393] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   12.569044] init: failsafe main process (1093) killed by TERM signal
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Running set_multiqueue.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Set channels for eth0 to 4.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Starting Google Accounts daemon.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Creating a new user account for me.
Aug 15 11:13:59 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Created user account me.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Created user account me.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Creating a new user account for bogdana.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Created user account bogdana.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Creating a new user account for aj.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Created user account aj.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Creating a new user account for asari.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Created user account asari.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf google-accounts: INFO Removing user packer.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   14.058420] floppy0: no floppy controllers found
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   14.139875] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   14.144484] Bridge firewalling registered
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   14.158506] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   14.196652] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   14.271792] Initializing XFRM netlink socket
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   14.279715] Netfilter messages via NETLINK v0.30.
Aug 15 11:14:00 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   14.282982] ctnetlink v0.93: registering with nfnetlink.
Aug 15 11:14:01 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   14.842301] random: nonblocking pool is initialized
Aug 15 11:14:04 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 11:14:04 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf cron[1635]: (CRON) INFO (pidfile fd = 3)
Aug 15 11:14:04 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 11:14:04 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf cron[1673]: (CRON) STARTUP (fork ok)
Aug 15 11:14:04 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf cron[1673]: (CRON) INFO (Running @reboot jobs)
Aug 15 11:14:04 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf acpid: starting up with netlink and the input layer
Aug 15 11:14:04 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf acpid: 1 rule loaded
Aug 15 11:14:04 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf acpid: waiting for events: event logging is off
Aug 15 11:14:04 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf haveged: haveged starting up
Aug 15 11:14:04 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   18.252447] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 11:14:09 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ntpd[1780]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 11:14:09 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ntpd[1781]: proto: precision = 0.172 usec
Aug 15 11:14:09 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ntpd[1781]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 11:14:09 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ntpd[1781]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
---
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf startup-script: INFO startup-script: job 1 at Wed Aug 15 14:24:00 2018
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf startup-script: INFO startup-script: Return code 0.
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf startup-script: INFO Finished running startup scripts.
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ec2: 
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ec2: #############################################################
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ec2: 1024 1d:ba:b3:3a:84:93:20:7f:84:87:9c:e9:d8:5e:d8:34  root@travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf (DSA)
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ec2: 256 dc:5f:5b:07:72:5a:b1:50:49:1e:c3:7c:80:56:b3:7d  root@travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf (ECDSA)
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ec2: 256 8f:88:9d:2b:28:98:48:b1:08:90:22:de:21:b5:e3:50  root@travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf (ED25519)
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ec2: 2048 f4:88:52:1f:52:f7:c4:e3:d9:44:67:48:70:da:34:2a  root@travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf (RSA)
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 15 11:14:10 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ec2: #############################################################
Aug 15 11:14:15 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf ntpdate[2175]: the NTP socket is in use, exiting
Aug 15 11:14:46 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [   59.672483] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 15 11:16:32 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [  166.198577] device veth8f55141 entered promiscuous mode
Aug 15 11:16:32 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [  166.198667] docker0: port 1(veth8f55141) entered forwarding state
Aug 15 11:16:32 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [  166.198673] docker0: port 1(veth8f55141) entered forwarding state
Aug 15 11:16:32 travis-job-3e023bba-9766-436f-a92e-0d6f4fc03baf kernel: [  166.19933433088 .
1601056 ./obj/build
1058044 ./src
995284 ./obj/build/x86_64-unknown-linux-gnu
773440 ./.git
---
151200 ./src/tools/clang
149116 ./src/llvm-emscripten/test
148684 ./obj/build/bootstrap/debug/incremental
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
134248 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f3vibug6i2-372shz-19p1no4jx31q5
103868 ./src/tools/lldb
98952 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
98520 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
93756 ./src/tools/clang/test
