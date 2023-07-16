plain
[00:49:09] ....................................................................................................
[00:49:11] ....................................................................................................
[00:49:14] ....................................................................................................
[00:49:16] ....................................................................................................
[00:49:18] ................................F.........................................................i.........
[00:49:25] ....................................................................................................
[00:49:28] ....................................................................................................
[00:49:30] ................................i...................................................................
[00:49:33] ....................................................................................................
---
[00:50:43] 
[00:50:43] ---- [ui] ui/feature-gates/feature-gate-panic-implementation.rs stdout ----
[00:50:43] diff of stderr:
[00:50:43] 
[00:50:43] 1 error[E0658]: #[panic_implementation] is an unstable feature (see issue #44489)
[00:50:43] -   --> $DIR/feature-gate-panic-implementation.rs:18:1
[00:50:43] +   --> $DIR/feature-gate-panic-implementation.rs:16:1
[00:50:43] 3    |
[00:50:43] 4 LL | #[panic_implementation] //~ ERROR #[panic_implementation] is an unstable feature (see issue #44489)
[00:50:43] 
[00:50:43] 
[00:50:43] The actual stderr differed from the expected stderr.
[00:50:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-panic-implementation/feature-gate-panic-implementation.stderr
[00:50:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-panic-implementation/feature-gate-panic-implementation.stderr
[00:50:43] To update references, rerun the tests and pass the `--bless` flag
[00:50:43] To only update this specific test, also pass `--test-args feature-gates/feature-gate-panic-implementation.rs`
[00:50:43] error: 1 errors occurred comparing output.
[00:50:43] status: exit code: 1
[00:50:43] status: exit code: 1
[00:50:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-panic-implementation.rs" "--c_implementation] is an unstable feature (see issue #44489)","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(panic_implementation)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: #[panic_implementation] is an unstable feature (see issue #44489)\n  --> /checkout/src/test/ui/feature-gates/feature-gate-panic-implementation.rs:16:1\n   |\nLL | #[panic_implementation] //~ ERROR #[panic_implementation] is an unstable feature (see issue #44489)\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(panic_implementation)] to the crate attributes to enable\n\n"}
[00:50:43] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:43] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[00:50:43] ------------------------------------------
[00:50:43] 
[00:50:43] thread '[ui] ui/feature-gates/feature-gate-panic-implementation.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:50:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:43] 
[00:50:43] 
[00:50:43] failures:
[00:50:43]     [ui] ui/feature-gates/feature-gate-panic-implementation.rs
[00:50:43] 
[00:50:43] test result: FAILED. 4107 passed;00] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000] kvm-clock: using sched offset of 1622259259 cycles
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000] Zone ranges:
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000]   Device   empty
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000] Movable zone start for each node
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000] Early memory node ranges
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    0.000000]   node   0: [mem 0x0000000000100 ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.396951] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.403257] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.410584] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.417004] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.440528] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.443805] vgaarb: loaded
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.446364] SCSI subsystem initialized
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.448457] libata version 3.00 loaded.
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.448483] ACPI: bus type USB registered
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.450599] usbcore: registered new interface driver usbfs
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.454718] usbcore: registered new interface driver hub
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.458303] usbcore: registered new device driver usb
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [    1.461862] ug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   10.363242] random: cloud-init: uninitialized urandom read (32 bytes read, 42 bits of entropy available)
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   10.510687] systemd-udevd[705]: starting version 204
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   10.642082] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   10.697184] intel_rapl: no valid rapl domains found in package 0
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   10.756298] ppdev: user-space parallel port driver
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   10.868446] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   10.930704] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   11.004070] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   11.176503] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 14 23:43:35 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   11.486477] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)f26-ea62b23eb276 google-accounts: INFO Created user account bogdana.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 google-accounts: INFO Creating a new user account for konstantin.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   13.733531] floppy0: no floppy controllers found
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   13.733855] work still pending
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   13.734422] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   13.738030] Bridge firewalling registered
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   13.750402] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 google-accounts: INFO Created user account konstantin.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 google-accounts: INFO Creating a new user account for carmen.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   13.807292] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 google-accounts: INFO Created user account carmen.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 google-accounts: INFO Creating a new user account for maria.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   13.877839] Initializing XFRM netlink socket
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   13.885247] Netfilter messages via NETLINK v0.30.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 kernel: [   13.888653] ctnetlink v0.93: registering with nfnetlink.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 google-accounts: INFO Created user account maria.
Aug 14 23:43:37 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 google-accounts: INFO Removing user packer.
Aug 14 23:43:39 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 cron[1712]: (CRON) INFO (pidfile fd = 3)
Aug 14 23:43:39 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 cron[1745]: (CRON) STARTUP (fork ok)
Aug 14 23:43:39 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 cron[1745]: (CRON) INFO (Running @reboot jobs)
Aug 14 23:43:39 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 23:43:39 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 23:43:39 travis-job-8e4f8145-6ef8-4aa9-8f26-ea62b23eb276 acpid: starting up with netlink and the input layer
Aug 14 23:43:39 travis-job-8e4f8145-6ef8-4aa9u/release/deps
112440 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
109004 ./src/llvm/test/CodeGen
98948 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
97904 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
