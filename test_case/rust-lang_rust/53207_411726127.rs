plain
[00:05:01]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:05:02] error: expected expression, found `i8`
[00:05:02]     --> libcore/num/mod.rs:348:21
[00:05:02]      |
[00:05:02] 348  | let n = ", $rot_op, $SelfT, ";
[00:05:02] ...
[00:05:02] ...
[00:05:02] 2012 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xai8" }
[00:05:02] 
[00:05:02] error: aborting due to previous error
[00:05:02] 
[00:05:02] error: Could not compile `core`.
[00:05:02] error: Could not compile `core`.
[00:05:02] 
[00:05:02] Caused by:
[00:05:02]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=bd44783aadae9ca1 -C extra-filename=-bd44783aadae9ca1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 1)
[00:05:02] warning: build failed, waiting for other jobs to finish...
[00:05:07] error: build failed
[00:05:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:05:07] expected success, got: exit code: 101
[00:05:07] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:05:07] travis_fold:end:stage0-std

[00:05:07] travis_time:end:stage0-std:start=1533814091074425425,finish=1533814097248831990,duration=6174406565


[00:05:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:07] Build completed unsuccessfully in 0:00:07
[00:05:07] make: *** [all] Error 1
[00:05:07] Makefile:28: recipe for target 'all' failed
0000f2800]
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] Using GB pages for direct mapping
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2383] evm: security.SMACK64MMAP
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.582890] evm: security.ima
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.583500] evm: security.capability
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.584697] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.586557] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.587886] pinctrl core: initialized pinctrl subsystem
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.589144] RTC time: 11:21:10, date: 08/09/18
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.590709] NET: Registered protocol family 16
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.599755] cpuidle: using governor ladder
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.611753] cpuidle: using governor menu
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.612467] PCCT header not found.
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.613420] ACPI: bus type PCI registered
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.614107] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.615367]OT_FOUND); disabling ASPM
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.699058] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.702249] PCI host bridge to bus 0000:00
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.703178] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.704509] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.705755] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.707528] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.709115] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.710305] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.710741] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.729654] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.745516] pci 0000:00:01.3: quirk: [io  0xb000-0xb03 0.842860] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.842863] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.843029] NetLabel: Initializing
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.844124] NetLabel:  domain hash size = 128
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.845342] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.846389] NetLabel:  unlabeled traffic allowed by default
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.847460] amd_nb: Cannot enumerate AMD northbridges
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.848335] clocksource: Switched to clocksource kvm-clock
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.857181] pnp: PnP ACPI init
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.858024] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.858098] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.858144] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    0.858195] pnp 00:03: Plug and Play ACPI device, IDs PNP0501
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.017962] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.041371] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.045191] Linux agpgart interface v0.103
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.048540] loop: module loaded
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.049141] libphy: Fixed MDIO Bus: probed
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.050421] tun: Universal TUN/TAP device driver, 1.6
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.051468] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.086806] PPP generic driver version 2.4.2
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.088227] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.089700] ehci-pci: EHCI PCI platform driver
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.090777] ehci-platform: EHCI generic platform driver
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.092130] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa1: version 1.0.3
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.111364] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.112596] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.115002] NET: Registered protocol family 10
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.116133] NET: Registered protocol family 17
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.117150] Key type dns_resolver registered
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.118655] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.120023] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.121339] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.122186] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.124055] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.126585] registered taskstats version 1
Aug  9 11:21:20 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [    3.127514] Loading compiled-in X.509 cer_netfilter if you need this.
Aug  9 11:21:21 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [   12.317091] Bridge firewalling registered
Aug  9 11:21:21 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [   12.329280] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 11:21:22 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 11:21:22 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [   12.352474] floppy0: no floppy controllers found
Aug  9 11:21:22 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [   12.352738] work still pending
Aug  9 11:21:22 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [   12.402284] Initializing XFRM netlink socket
Aug  9 11:21:22 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [   12.413952] Netfilter messages via NETLINK v0.30.
Aug  9 11:21:22 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [   12.416424] ctnetlink v0.93: registering with nfnetlink.
Aug  9 11:21:45 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ntpdate[1791]: adjust time server 169.254.169.254 offset 0.003773 sec
Aug  9 11:21:51 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ntpd[1827]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 11:21:51 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ntpd[1828]: proto: precision = 0.175 usec
Aug  9 11:21:51 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ntpd[1828]: ntp_io: estimated max descriptors: 1ode 0.
Aug  9 11:21:51 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ntpd[1828]: ntp_io: estimated max descriptors: 1ode 0.
Aug  9 11:21:52 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d startup-script: INFO Finished running startup scripts.
Aug  9 11:21:52 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ec2: 
Aug  9 11:21:52 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ec2: #############################################################
Aug  9 11:21:52 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 11:21:52 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ec2: 1024 fc:94:9f:99:d2:16:f1:58:9a:5f:2a:46:7f:7d:18:62  root@travis-job-21277e65-17be-4923-ab44-fa18e72bf69d (DSA)
Aug  9 11:21:52 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ec2: 256 50:ae:f7:39:3a:81:7a:98:59:93:08:14:13:2a:7c:79  root@travis-job-21277e65-17be-4923-ab44-fa18e72bf69d (ECDSA)
Aug  9 11:21:52 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ec2: 256 ca:34:a8:29:e8:b4:39:93:17:69:8e:11:16:80:c4:09  root@travis-job-21277e65-17be-4923-ab44-fa18e72bf69d (ED25519)
Aug  9 11:21:52 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ec2: 2048 f3:32:97:99:42:28:94:03:34:99:31:ac:7f:ca:d1:d0  root@travis-job-21277e65-17be-4923-ab44-fa18e72bf69d (RSA)
Aug  9 11:21:52 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 11:21:52 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d ec2: #############################################################
Aug  9 11:23:09 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [  119.533490] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 11:24:10 travis-job-21277e65-17be-4923-ab44-fa18e72bf69d kernel: [  181.018025] dev         768M     0  768M   0% /var/ramfs
1102016 ./obj
1101980 ./obj/build
792908 ./src
568596 ./.git
---
158448 ./.git/modules/src
149132 ./src/llvm-emscripten/test
147700 ./obj/build/bootstrap/debug/incremental
133268 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
133264 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f3owdynsna-12x5jvr-10j4kdvcqyom
98948 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
78524 ./.git/modules/src/tools
72140 ./src/llvm/lib
65428 ./src/llvm-emscripten/test/CodeGen
