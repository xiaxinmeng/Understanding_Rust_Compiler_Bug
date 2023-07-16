plain
[00:12:45]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:12:45]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:12:45]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:12:46]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:12:50] error[E0599]: no method named `codemap` found for type `&rustc::session::Session` in the current scope
[00:12:50]    --> librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:188:27
[00:12:50]     |
[00:12:50] 188 |         let cm = tcx.sess.codemap();
[00:12:50] 
[00:12:50] 
[00:12:50] error[E0599]: no method named `codemap` found for type `&rustc::session::Session` in the current scope
[00:12:50]    --> librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:608:26
[00:12:50]     |
[00:12:50] 608 |                 tcx.sess.codemap().end_point(span),
[00:12:50] 
[00:12:56] error: aborting due to 2 previous errors
[00:12:56] 
[00:12:56] For more information about this error, try `rustc --explain E0599`.
[00:12:56] For more information about this error, try `rustc --explain E0599`.
[00:12:56] error: Could not compile `rustc_mir`.
[00:12:56] 
[00:12:56] Caused by:
[00:12:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=ff181b58e43e5a7e -C extra-filename=-ff181b58e43e5a7e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-b8f9e6fb5ae336d7.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0a515e87c8afea9e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f501f9d595863bbf.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-c6e8cf18dad58451.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-b308e2da12bfc8a3.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-165c205e2819b15f.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8b624a6d6082b2ff.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-89eed8215142aadd.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-64bbe8e4870170a3.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-3b51f50aecba154c.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-01a673445b66da02/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/out` (exit code: 1)
[00:14:04] error: build failed
[00:14:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:04] expected success, got: exit code: 101
[00:14:04] expected success, got: exit code: 101
[00:14:04] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:14:04] travis_fold:end:stage0-rustc

[00:14:04] travis_time:end:stage0-rustc:start=1533749395482492488,finish=1533749940304970284,duration=544822477796


[00:14:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:04] Build completed unsuccessfully in 0:09:59
[00:14:04] Makefile:28: recipe for target 'all' failed
[00:14:04] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e270eac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Aug  8 17:39:00 UTC 2018
BIOS EBDA area
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000] console [ttyS0] enabled
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.437261] Calib50] Initializing cgroup subsys memory
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.483204] Initializing cgroup subsys devices
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.484753] Initializing cgroup subsys freezer
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.486103] Initializing cgroup subsys net_cls
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.487358] Initializing cgroup subsys perf_event
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.488912] Initializing cgroup subsys net_prio
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.490679] Initializing cgroup subsys hugetlb
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.492341] Initializing cgroup subsys pids
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.493950] CPU: Physical Processor ID: 0
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.495162] CPU: Processor Core ID: 0
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.498255] mce: CPU supports 32 MCE banks
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.500538] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.502454] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    0.507145] Freeing SMP alternatives memory: 32K
l: [    1.049398] ACPI: bus type USB registered
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.050739] usbcore: registered new interface driver usbfs
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.052676] usbcore: registered new interface driver hub
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.054158] usbcore: registered new device driver usb
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.055790] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.057753] dmi: Firmware registration failed.
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.059174] PCI: Using ACPI for IRQ routing
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.060315] PCI: pci_cache_line_size set to 64 bytes
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.060415] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.060416] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.060532] NetLabel: Initializing
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.061477] NetLabel:  domain hash size = 128
Aug  8 17:23:52 travis-job-64ca96c9-fa30-4895-a2e4-a6cca39ff47b kernel: [    1.062977] NetLabel:  protocols = UNLABELED CIPSOv4
