plain
[00:21:33]    Compiling chalk-macros v0.1.0
[00:21:35]    Compiling miniz-sys v0.1.10
[00:21:37]    Compiling backtrace-sys v0.1.23
[00:21:38]    Compiling humantime v1.1.1
x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern chalk_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_macros-1ec71b9a040fef8a.rlib --extern rustc_hash=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-9bd093322605ad94.rlib --cap-lints allow` (exit code: 101)
syslog
syslog
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] MTRR variable ranges enabled:
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   0 base 0000C0000000 mask 3FFFC0000000 uncachable
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   1 disabled
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   2 disabled
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   3 disabled
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   4 disabled
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   5 disabled
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   6 disabled
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   7 disabled
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] Base -b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   Device   empty
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] Movable zone start for each node
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] Early memory node ranges
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] Policy zone: Normal
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [   ash size = 128
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.951359] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.952604] NetLabel:  unlabeled traffic allowed by default
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.954711] amd_nb: Cannot enumerate AMD northbridges
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.955893] clocksource: Switched to clocksource kvm-clock
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.964027] pnp: PnP ACPI init
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.964705] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.964776] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.964821] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.964870] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.964912] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.964951] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    0.964995] pnp 00:06: Plug and Play ACPI d14a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.150555] ACPI: Sleep Button [SLPF]
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.151625] GHES: HEST is not enabled!
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.154373] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.155324] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.161138] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.162162] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.169686] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.193880] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.217897] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.241621] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.265226] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.268911] Linux agpgart interfac0814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.401535] Freeing unused kernel memory: 1956K
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.402695] Freeing unused kernel memory: 92K
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.418728] systemd-udevd[120]: starting version 204
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.487201] scsi host0: Virtio SCSI HBA
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.495255] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.501424] AVX version of gcm_enc/dec engaged.
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.502942] AES CTR mode by8 optimization enabled
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.540432] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.546992] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.547027] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.547029] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    3.547225] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbnel: [    9.756952] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    9.821742] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [    9.985130] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [   10.262502] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [   10.332634] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [   10.414521] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [   10.446606] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [   10.732141] init: failsafe main process (1093) killed by TERM signal
Aug  7 07:19:23 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 07:19:24 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 instance-setup: INFO Running set_multiqueue.
Aug  7 07:19:24 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 instance-setup: INFO Set channels f4-4860-930e-f5dbcd91ea33 ec2: 256 01:80:76:36:3b:23:7a:f2:4a:68:ad:0f:b2:98:d0:5e  root@travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 (ECDSA)
Aug  7 07:19:55 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 ec2: 256 35:76:e4:1f:63:fb:a3:21:71:0b:6c:f8:a4:53:9b:e5  root@travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 (ED25519)
Aug  7 07:19:55 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 ec2: 2048 b1:d7:55:d0:f8:1b:28:2c:6e:0d:55:c4:a6:58:10:c8  root@travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 (RSA)
Aug  7 07:19:55 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 07:19:55 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 ec2: #############################################################
Aug  7 07:20:24 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [   71.948288] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 07:22:42 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [  209.629599] device veth444d184 entered promiscuous mode
Aug  7 07:22:42 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [  209.719290] cgroup: docker-runc (4779) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 07:22:42 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [  209.719294] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 07:22:42 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [  209.783120] eth0: renamed from vethf8fcc78
Aug  7 07:22:42 travis-job-b950814a-ced4-4860-930e-f5dbcd91ea33 kernel: [  209.820148] IPv6: ADDRCON/include
14456 ./src/llvm/test/MC/Disassembler
14164 ./src/tools/lld
travis_time:end:12b32f3e:start=1533627753996639663,finish=1533627754263515156,duration=266875493
travis_fold:end:after_failure.2
---
travis_time:end:010c8412:start=1533627754290454112,finish=1533627754299210648,duration=8756536
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b68c80c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
