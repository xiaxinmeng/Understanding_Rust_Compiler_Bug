plain
[00:23:08]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:23:13]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:24:38]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:24:48]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:24:53] error: anonymous parameters are deprecated and will be removed in the next edition.
[00:24:53]    --> librustc/lint/mod.rs:276:39
[00:24:53]     |
[00:24:53] 276 |             expand_lint_pass_methods!(&LateContext<'a, $hir>, [$($methods)*]);
[00:24:53]     |                                       ^^^^^^^^^^^^^^^^^^^^^^ help: Try naming the parameter or explicitly ignoring it: `_: &LateContext<'a, $hir>`
[00:24:53] ...
[00:24:53] 281 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:24:53]     | ------------------------------------------------------- in this macro invocation
[00:24:53]     |
[00:24:53]     = note: `-D anonymous-parameters` implied by `-D warnings`
[00:24:53]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:24:53] 
[00:24:53] 
[00:24:53] error: anonymous parameters are deprecated and will be removed in the next edition.
[00:24:53]    --> librustc/lint/mod.rs:276:39
[00:24:53]     |
[00:24:53] 276 |             expand_lint_pass_methods!(&LateContext<'a, $hir>, [$($methods)*]);
[00:24:53]     |                                       ^^^^^^^^^^^^^^^^^^^^^^ help: Try naming the parameter or explicitly ignoring it: `_: &LateContext<'a, $hir>`
[00:24:53] ...
[00:24:53] 281 | late_lint_methods!(declare_late_lint_pass, [], ['tcx]);
[00:24:53]     | ------------------------------------------------------- in this macro invocation
[00:24:53]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:24:53]     = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
[00:24:53] 
[00:25:25] error: aborting due to 2 previous errors
[00:25:25] error: aborting due to 2 previous errors
[00:25:25] 
[00:25:25] error: Could not compile `rustc`.
[00:25:25] 
[00:25:25] Caused by:
[00:25:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=3be8fb23eaf1b952 -C extra-filename=-3be8fb23eaf1b952 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-4701371fdd2d83ac.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-8091a0e1fe9f036b.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9b04b6981047a227.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-da03a033b1a119c5.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-30c883ec86c7b095.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/0a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706fnecessary, use "pci=nocrs" and report a bug
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.022998] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.026517] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.030343] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.034313] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.043184] PCI host bridge to bus 0000:00
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.045470] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.049516] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.053498] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.058026] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.062880] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.066757] pci 0706f59cebcdb kernel: [    1.299923] PCI: Using ACPI for IRQ routing
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.302800] PCI: pci_cache_line_size set to 64 bytes
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.302913] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.302915] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.303058] NetLabel: Initializing
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.305456] NetLabel:  domain hash size = 128
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.307746] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.311065] NetLabel:  unlabeled traffic allowed by default
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.315083] amd_nb: Cannot enumerate AMD northbridges
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.319769] clocksource: Switched to clocksource kvm-clock
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.329865] pnp: PnP ACPI init
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.331680] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    1.331762] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 11 16-bd0a-706f59cebcdb kernel: [    3.556084] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.559831] intel_idle: does not run on family 6 model 45
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.559940] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.563716] ACPI: Power Button [PWRF]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.565805] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.569973] ACPI: Sleep Button [SLPF]
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.572880] GHES: HEST is not enabled!
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.577828] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.581552] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.593506] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.596977] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.609603] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 11 16:39:58 travis-job-f50a60d4-6f-bd0a-706f59cebcdb kernel: [    3.963205] EDD information not available.
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.965960] PM: Hibernation image not present or could not be loaded.
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.967502] Freeing unused kernel memory: 1496K
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.970633] Write protecting the kernel read-only data: 14336k
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.975155] Freeing unused kernel memory: 1956K
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.978065] Freeing unused kernel memory: 92K
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    3.998383] systemd-udevd[118]: starting version 204
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    4.050845] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    4.072585] scsi host0: Virtio SCSI HBA
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    4.079942] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    4.093336] AVX version of gcm_enc/dec engaged.
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    4.096613] AES CTR mode by8 optimization enabled
Aug 11 16:39:58 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [    4.181985] sd 0:02 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb cron[1653]: (CRON) STARTUP (fork ok)
Aug 11 16:40:02 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 16:40:02 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 16:40:02 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb cron[1653]: (CRON) INFO (Running @reboot jobs)
Aug 11 16:40:02 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb acpid: starting up with netlink and the input layer
Aug 11 16:40:02 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb acpid: 1 rule loaded
Aug 11 16:40:02 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb acpid: waiting for events: event logging is off
Aug 11 16:40:02 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb haveged: haveged starting up
Aug 11 16:40:02 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [   15.693118] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 16:40:07 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb ntpd[1753]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 11 16:40:07 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb ntpd[1754]: proto: precision = 0.100 usec
Aug 11 16:40:07 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb ntpd[1754]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 16:40:07 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb ntpd[1754]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 16:40:07 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb ntpd[1754]: Listen and drop on 0 v4wildcard 0cuous mode
Aug 11 16:43:31 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [  224.614863] cgroup: docker-runc (4772) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 11 16:43:31 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [  224.614866] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 11 16:43:31 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [  224.682362] eth0: renamed from vethe3f0c6f
Aug 11 16:43:31 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [  224.728043] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 11 16:43:31 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [  224.729431] docker0: port 1(veth2ab0d18) entered forwarding state
Aug 11 16:43:31 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [  224.729448] docker0: port 1(veth2ab0d18) entered forwarding state
Aug 11 16:43:31 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb kernel: [  224.729481] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 11 16:43:34 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb ntpd[1754]: Listen normally on 5 docker0 fe80::42:78ff:fea7:abaf UDP 123
Aug 11 16:43:34 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb ntpd[1754]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 11 16:43:34 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cebcdb ntpd[1754]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 11 16:43:34 travis-job-f50a60d4-234b-4d6f-bd0a-706f59cwn-linux-gnu/stage0-codegen
14984 ./src/test/run-pass
14868 ./src/llvm/include
14524 ./src/llvm/include/llvm
---
travis_time:end:1f0e2058:start=1534007277244997877,finish=1534007277253225505,duration=8227628
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08ebd848
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:cr
