plain
[00:07:29]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:33] error[E0425]: cannot find value `len` in this scope
[00:07:33]    --> librustc/traits/error_reporting.rs:473:30
[00:07:33]     |
[00:07:33] 473 |                           if len > 5 {
[00:07:33] 
[00:07:33] error[E0425]: cannot find value `len` in this scope
[00:07:33]    --> librustc/traits/error_reporting.rs:474:58
[00:07:33]     |
[00:07:33]     |
[00:07:33] 474 |                               format!("\nand {} others", len - 4)
[00:07:33] 
[00:07:58] error: aborting due to 2 previous errors
[00:07:58] 
[00:07:58] For more information about this error, try `rustc --explain E0425`.
[00:07:58] For more information about this error, try `rustc --explain E0425`.
[00:07:58] error: Could not compile `rustc`.
[00:07:58] 
[00:07:58] Caused by:
[00:07:58]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=0dae47d5f57c38d7 -C extra-filename=-0dae47d5f57c38d7 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-b8f9e6fb5ae336d7.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-4df0a88f8df46534.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-e5f2yon_core-d60152ee716e8998.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-165c205e2819b15f.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8b624a6d6082b2ff.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-89eed8215142aadd.so --extern rustc_fs_util=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_fs_util-3f97c3ca5071d1e2.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-6a496b98b1d59ff3.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-64bbe8e4870170a3.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-3b51f50aecba154c.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-7a923cd3dbf409d7.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-01a673445b66da02/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/out` (exit code: 1)
[00:07:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:58] expected success, got: exit code: 101
[00:07:58] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:07:58] travis_fold:end:stage0-rustc

[00:07:58] travis_time:end:stage0-rustc:start=1534306141347595171,finish=1534306307540400972,duration=166192805801


[00:07:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:58] Build completed unsuccessfully in 0:03:43
[00:07:58] Makefile:28: recipe for target 'all' failed
[00:07:58] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:070e263a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:023ab4ac
$ sudo tail -n 500 /var/log/syslog
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   5 disabled
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   6 disabled
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   7 disabled
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992efffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] kvm-clock: using sched offset of 1826733265 cycles
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Zone ranges:
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   Device   empty
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Movable zone start for each node
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Early memory node ranges
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] ACPI: IR0xffffffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Policy zone: Normal
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.000000] console [ttyS0] enabled
A262144 bytes)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.537744] Initializing cgroup subsys io
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.539477] Initializing cgroup subsys memory
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.542189] Initializing cgroup subsys devices
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.544495] Initializing cgroup subsys freezer
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.547278] Initializing cgroup subsys net_cls
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.548820] Initializing cgroup subsys perf_event
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.550986] Initializing cgroup subsys net_prio
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.553186] Initializing cgroup subsys hugetlb
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.554868] Initializing cgroup subsys pids
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.556628] CPU: Physical Processor ID: 0
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.558762] CPU: Processor Core ID: 0
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.560250] mce: CPU supports 32 MCE banks
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.562022] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.563957] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.569167] Freeing SMP alternatives memory: 32K
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.583028] ftrace: allocating 32185 entries in 126 pages
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.644734] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.647966] smpboot: Max logical packages: 2
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.650459] x2apic enabled
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.653846] Switched APIC routing to physical x2apic.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.659670] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.769583] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.773123] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.777962] x86: Booting SMP configuration:
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.779459] .... node  #0, CPUs:      #1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.781383] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.786333]  #2
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.787228] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.792012]  #3
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.792892] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.797358] x86: Booted up 1 node, 4 CPUs
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.798891] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.803135] devtmpfs: initialized
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.808705] evm: security.selinux
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.810164] evm: security.SMACK64
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.811204] evm: security.SMACK64EXEC
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.812572] evm: security.SMACK64TRANSMUTE
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.814276] evm: security.SMACK64MMAP
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.815670] evm: security.ima
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb on space under this bridge.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.975671] PCI host bridge to bus 0000:00
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.976881] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.979088] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.981886] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.985390] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.988363] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.990543] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    0.991051] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.017933] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.049743] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.053892] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.066075] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.076055] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.102404] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.113952] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.123532] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.150813] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.155786] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.161352] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.166779] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.172070] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.195823] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.198530] vgaarb: loaded
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.199783] SCSI subsystem initialized
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.201257] libata version 3.00 loaded.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.201285] ACPI: bus type USB registered
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.203574] usbcore: registered new interface driver usbfs
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.205876] usbcore: registered new interface driver hub
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.207892] usbcore: registered new device driver usb
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.209754] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.212180] dmi: Firmware registration failed.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.213985] PCI: Using ACPI for IRQ routing
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.216173] PCI: pci_cache_line_size set to 64 bytes
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.216288] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.216292] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.216457] NetLabel: Initializing
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel:85] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.240874] pnp: PnP ACPI: found 7 devices
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.250278] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.253710] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.253713] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.253715] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.253717] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.253760] NET: Registered protocol family 2
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.255940] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.259712] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.262731] TCP: Hash tables configured (established 131072 bind 65536)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.265230] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.267349] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.271903] NET: Registered protocol family 1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.273899] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.276940] PCI: CLS 0 bytes, default 64
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    1.277030] Unpacking initramfs...
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.418100] Freeing initrd memory: 21432K
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.419163] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.420313] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.422230] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.423792] hw unit of domain pp0-core 2^-0 Joules
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.424652] hw unit of domain package 2^-0 Joules
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.425389] hw unit of domain dram 2^-0 Joules
Aug 1locating IMA MOK and blacklist keyrings.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.444811] Key type asymmetric registered
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.445702] Asymmetric key parser 'x509' registered
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.446647] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.448358] io scheduler noop registered
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.449106] io scheduler deadline registered (default)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.450085] io scheduler cfq registered
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.451229] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.452399] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.453641] intel_idle: does not run on family 6 model 45
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.453767] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.455735] ACPI: Power Button [PWRF]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.456456] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.457981] ACPI: Sleep Button [SLPF]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.459028] GHES: HEST is not enabled!
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.462366] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.463576] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.470646] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.471753] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.478189] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.501242] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.525793] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.550519] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.575068] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [ db kernel: [    3.670492] NET: Registered protocol family 10
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.672247] NET: Registered protocol family 17
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.673995] Key type dns_resolver registered
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.675655] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.676802] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.678616] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.680621] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.682133] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.685699] registered taskstats version 1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.686988] Loading compiled-in X.509 certificates
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.688959] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.692404] zswap: loaded using pool lzo/zbud
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.695966] Key type trusted registered
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.700889] Key type encrypted registered
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.702161] ima: No TPM chip found, activating TPM-bypass!
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.703935] evm: HMAC attrs: 0x1
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.705530]   Magic number: 14:521:9
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.706741] memory memory53: hash matches
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.708266] rtc_cmos 00:00: setting system clock to 2018-08-15 04:02:50 UTC (1534305770)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.711311] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.713237] EDD information not available.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.714824] PM: Hibernation image not present or could not be loaded.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.716674] Freeing unused kernel memory: 1496K
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.717580] Write protecting the kernel read-only data: 14336k
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    3.719883] Freeing unused kernel memory: 1956K
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-262: sse2x4   xor()  8546 MB/s
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.307137] raid6: using algorithm sse2x4 gen() 12333 MB/s
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.309320] raid6: .... xor() 8546 MB/s, rmw enabled
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.312061] raid6: using ssse3x2 recovery algorithm
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.315863] xor: automatically using best checksumming function:
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.356148]    avx       : 21728.000 MB/sec
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.373830] Btrfs loaded
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.436836] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.439642] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.520138] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.529284] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.532395] EXT4-fs (sda1): recovery complete
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.540432] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.799759] random: init: uninitialized urandom read (12 bytes read, 22 bits of entropy available)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    8.943492] random: mountall: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    9.012127] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    9.268884] random: cloud-init: uninitialized urandom read (32 bytes read, 33 bits of entropy available)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [    9.966810] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   10.134909] systemd-udevd[703]: starting version 204
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   10.274682] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   10.330074] intel_rapl: no valid rapl domains found in package 0
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   10.390142] intel_rapl: no valid rapl domains found in package 0
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   10.391582] ppdev: user-space parallel port driver
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   10.503839] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   10.567556] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   10.647038] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   10.821380] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   11.101261] random: mktemp: uninitialized urandom read (12 bytes read, 55 bits of entropy available)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   11.189215] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   11.285011] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   11.336414] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   11.789608] init: failsafe main process (1094) killed by TERM signal
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb instance-se Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 04:02:58 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 04:02:59 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 04:02:59 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 04:02:59 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-accounts: INFO Starting Google Accounts daemon.
Aug 15 04:02:59 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-accounts: INFO Creating a new user account for me.
Aug 15 04:02:59 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 04:02:59 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-accounts: INFO Created user account me.
Aug 15 04:02:59 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-accounts: INFO Creating a new user account for bogdana.
Aug 15 04:03:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-clock-skew: INFO Synced system time with hardware clock.
Aug 15 04:03:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-accounts: INFO Created user account bogdana.
Aug 15 04:03:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-accounts: INFO Creating a new user account for aj.
Aug 15 04:03:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-accounts: INFO Created user account aj.
Aug 15 04:03:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-accounts: INFO Creating a new user account for asari.
Aug 15 04:03:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-accounts: INFO Created user account asari.
Aug 15 04:03:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb google-accounts: INFO Removing user packer.
Aug 15 04:03:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   13.162704] random: nonblocking pool is initialized
Aug 15 04:03:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   13.296055] bridge: automatic filtering via arp/ip/ip6tables hasstem was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 04:03:01 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 04:03:01 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb acpid: starting up with netlink and the input layer
Aug 15 04:03:01 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb acpid: 1 rule loaded
Aug 15 04:03:01 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb acpid: waiting for events: event logging is off
Aug 15 04:03:01 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb haveged: haveged starting up
Aug 15 04:03:01 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   14.575063] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1773]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: proto: precision = 0.106 usec
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: Listen normally on 3 eth0 10.20.0.3 UDP 123
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: peers refreshed
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: Listening on routing socket on fd #21 for interface updates
Aug 15 04:03:06 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [   19.789767] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 04:03:07 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb startup-script: INFO Found startup-script in metadata.
Aug 15 04:03:07 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 15 04:03:07 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb startup-script: INFO startup-script: job 1 at Wed Aug 15 07:13:00 2018
Aug 15 04:03:07 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb startup-script: INFO startup-script: Return code 0.
Aug 15 04:03:07 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb startup-script: INFO startup-script: Return code 0.
Aug 15 04:03:07 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb startup-script: INFO Finished running startup scripts.
Aug 15 04:03:07 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ec2: 
Aug 15 04:03:07 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ec2: #############################################################
Aug 15 04:03:07 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ec2: -----BEGIN SSH HOST KEY 4:05:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: peers refreshed
Aug 15 04:05:00 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb ntpd[1774]: new interface(s) found: waking up resolver
Aug 15 04:05:12 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [  145.288835] docker0: port 1(veth01d9fa3) entered forwarding state
Aug 15 04:11:47 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [  540.382320] veth776bbe6: renamed from eth0
Aug 15 04:11:47 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [  540.419942] docker0: port 1(veth01d9fa3) entered disabled state
Aug 15 04:11:47 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [  540.452962] docker0: port 1(veth01d9fa3) entered disabled state
Aug 15 04:11:47 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [  540.454930] device veth01d9fa3 left promiscuous mode
Aug 15 04:11:47 travis-job-2dd6d4ce-f4cc-4e64-8c2e-2624992e2edb kernel: [  540.454933] docker0: port 1(veth01d9fa3) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0ab31af2
---
11760 ./src/llvm/test/MC/X86
10956 ./src/llvm/test/MC/Disassembler/AMDGPU
10792 ./.git/modules/src/tools/clippy
travis_time:end:0ab31af2:start=1534306307893983750,finish=1534306308141018620,duration=247034870
travis_fold:ex-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:04b74d36
$ dmesg | grep -i kill
