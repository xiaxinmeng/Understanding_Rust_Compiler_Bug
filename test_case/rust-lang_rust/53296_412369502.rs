plain
[00:46:06] ....................................................................................................
[00:46:08] ....................................................................................................
[00:46:11] ....................................................................................................
[00:46:14] ....................................................................................................
[00:46:17] ...............iiiiiiiii............................................................................
[00:46:22] ....................................................................................................
[00:46:26] .....................i..............................................................................
[00:46:29] ................................i...................................................................
[00:46:32] ....................................................................................................
---
[00:52:23] ....................................................................................................
[00:52:26] ....................................................................................................
[00:52:30] ...........................................................................................i........
[00:52:32] ...................................................ii.iii...........................................
[00:52:35] .............................................................F.....................i.F..............
[00:52:42] ....................................................................................................
[00:52:45] ............................................................i.......................................
[00:52:48] .......................................................i..ii........................................
[00:52:51] ....................................................................................................
---
[00:53:10] 
[00:53:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:53:10] 
[00:53:10] 
[00:53:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-pa0001000-0x00000003ffffffff]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.0000er
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.483769] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.495855] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.502470] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.504899] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.507442] Initializing cgroup subsys io
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.508793] Initializing cgroup subsys memory
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.510448] Initializing cgroup subsys devices
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.512290] Initializing cgroup subsys freezer
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.513640] Initializing cgroup subsys net_cls
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.514911] Initializing cgroup subsys perf_event
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.516732] Initializing cgroup subsys net_prio
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.518350] Initializing cgroup subsys hugetlb
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4s-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.772482] evm: security.SMACK64
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.773746] evm: security.SMACK64EXEC
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.775057] evm: security.SMACK64TRANSMUTE
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.776385] evm: security.SMACK64MMAP
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.777793] evm: security.ima
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.778857] evm: security.capability
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.780452] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.783923] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.786442] pinctrl core: initialized pinctrl subsystem
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.788288] RTC time: 19:33:05, date: 08/12/18
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.790993] NET: Registered protocol family 16
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.804116] cpuidle: using governor ladder
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.816145] cpuidle: using governor menu
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-8935243:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.910461] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.912664] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.915179] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.917130] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.921466] PCI host bridge to bus 0000:00
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.922992] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.925613] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.927979] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.930348] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.932998] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    0.934819] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 12 ernel: [    1.134318] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.134366] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.134408] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.134446] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.134485] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.134680] pnp: PnP ACPI: found 7 devices
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.143218] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.146195] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.146198] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.146200] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.146201] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    1.146243] NET: Registeredpress Hot Plug Controller Driver version: 0.4
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.442075] intel_idle: does not run on family 6 model 62
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.442173] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.444635] ACPI: Power Button [PWRF]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.446143] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.448721] ACPI: Sleep Button [SLPF]
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.450724] GHES: HEST is not enabled!
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.455135] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.457217] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.465872] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.468024] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.478216] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [    3.502057] 00:033524564f47 instance-setup: INFO Running set_multiqueue.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO Set channels for eth0 to 4.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f45-4fbd-a4de-893524564f47 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO Creating a new user account for me.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO Created user account me.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO Creating a new user account for henrik.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO Created user account henrik.
Aug 12 19:33:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO Creating a new user account for emma.
Aug 12 19:33:17 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO Created user account emma.
Aug 12 19:33:17 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO Creating a new user account for igor.
Aug 12 19:33:17 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO Created user account igor.
Aug 12 19:33:17 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [   12.319748] random: nonblocking pool is initialized
Aug 12 19:33:17 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 12 19:33:17 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO Created user account konstantinhaase.
Aug 12 19:33:17 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 google-accounts: INFO CreatingFO Finished running startup scripts.
Aug 12 19:33:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ec2: 
Aug 12 19:33:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ec2: #############################################################
Aug 12 19:33:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 12 19:33:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ec2: 1024 da:e9:ff:57:10:9c:9e:28:e2:51:c6:bb:c9:e7:ce:d7  root@travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 (DSA)
Aug 12 19:33:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ec2: 256 a0:d3:1a:f3:ff:29:f5:53:3f:d1:20:b3:d7:8a:a5:16  root@travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 (ECDSA)
Aug 12 19:33:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ec2: 256 e6:3c:19:bf:7f:a6:0e:f2:3b:9a:81:ac:7c:85:19:06  root@travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 (ED25519)
Aug 12 19:33:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ec2: 2048 9f:ae:bf:93:4f:e7:5d:c9:8f:e5:59:31:d2:4a:80:00  root@travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 (RSA)
Aug 12 19:33:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 12 19:33:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ec2: #############################################################
Aug 12 19:33:33 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ntpdate[2248]: the NTP socket is in use, exiting
Aug 12 19:34:12 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [   67.199985] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 12 19:35:18 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 ke905c9-8005-4fbd-a4de-893524564f47 kernel: [ 3122.483668] a[18953]: segfault at 0 ip 00005583cc471463 sp 00007ffda921f650 error 6 in a[5583cc46e000+5000]
Aug 12 20:25:16 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [ 3131.053162] a[19711]: segfault at 1 ip 00005559d092db8c sp 00007ffd325ab310 error 6 in a[5559d092b000+4000]
Aug 12 20:25:20 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [ 3134.825264] traps: a[20083] trap invalid opcode ip:562361b9c42c sp:7ffc1585ecd0 error:0 in a[562361b99000+7000]
Aug 12 20:27:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [ 3258.308638] veth696f1dd: renamed from eth0
Aug 12 20:27:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [ 3258.354509] docker0: port 1(veth96cf199) entered disabled state
Aug 12 20:27:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [ 3258.382829] docker0: port 1(veth96cf199) entered disabled state
Aug 12 20:27:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [ 3258.384792] device veth96cf199 left promiscuous mode
Aug 12 20:27:23 travis-job-7d2905c9-8005-4fbd-a4de-893524564f47 kernel: [ 3258.384796] docker0: port 1(veth96cf199) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0a722a6e
