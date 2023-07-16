plain
[00:13:07]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:13:18] error[E0308]: mismatched types
[00:13:18]    --> librustc_mir/interpret/memory.rs:687:57
[00:13:18]     |
[00:13:18] 687 |                     ptr::copy(src_bytes, dest_bytes.add(size.bytes() * i), size.bytes() as usize);
[00:13:18] 
[00:13:18] error[E0308]: mismatched types
[00:13:18]    --> librustc_mir/interpret/memory.rs:691:72
[00:13:18]     |
[00:13:18]     |
[00:13:18] 691 |                     ptr::copy_nonoverlapping(src_bytes, dest_bytes.add(size.bytes() * i), size.bytes() as usize);
[00:13:18] 
[00:13:19] error: aborting due to 2 previous errors
[00:13:19] 
[00:13:19] For more information about this error, try `rustc --explain E0308`.
[00:13:19] For more information about this error, try `rustc --explain E0308`.
[00:13:19] error: Could not compile `rustc_mir`.
[00:13:19] 
[00:13:19] Caused by:
[00:13:19]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=79a31f315d628970 -C extra-filename=-79a31f315d628970 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-b8f9e6fb5ae336d7.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0a515e87c8afea9e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f501f9d595863bbf.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-c6e8cf18dad58451.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-0dae47d5f57c38d7.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-165c205e2819b15f.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8b624a6d6082b2ff.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-89eed8215142aadd.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-64bbe8e4870170a3.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-3b51f50aecba154c.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-01a673445b66da02/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/out` (exit code: 1)
[00:13:19] warning: build failed, waiting for other jobs to finish...
484-4ff5-b0bc-b251adfd505e kernel: [    1.203440] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    1.207126] NET: Registered protocol family 1
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    1.208662] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    1.210748] PCI: CLS 0 bytes, default 64
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    1.210811] Unpacking initramfs...
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    3.419195] Freeing initrd memory: 21432K
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    3.420076] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    3.421101] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    3.423042] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    3.424332] hw unit of domain pp0-core 2^-0 Joules
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    3.425007] hw unit of domain package 2^-0 Joules
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [    3.425877] hw unit of domain dram 2^-0 Joules
Aug 14 04:25:43 travis-job-9a42d39e-c484-4ff5-b0bc-bered disabled state
Aug 14 04:28:29 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [  177.839314] cgroup: docker-runc (4829) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 04:28:29 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [  177.839317] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 04:28:29 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [  177.923815] eth0: renamed from vetha19e83f
Aug 14 04:28:29 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [  177.967571] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 04:28:29 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [  177.968965] docker0: port 1(veth03a3bfc) entered forwarding state
Aug 14 04:28:29 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [  177.968985] docker0: port 1(veth03a3bfc) entered forwarding state
Aug 14 04:28:29 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e kernel: [  177.969012] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 04:28:32 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e ntpd[1844]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 14 04:28:32 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e ntpd[1844]: Listen normally on 6 docker0 fe80::42:3eff:fe03:6c90 UDP 123
Aug 14 04:28:32 travis-job-9a42d39e-c484-4ff5-b0bc-b251adfd505e ntpd[1844]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 04:28:32 travis-job-9a42d39e-c484-4ff5-b0b
