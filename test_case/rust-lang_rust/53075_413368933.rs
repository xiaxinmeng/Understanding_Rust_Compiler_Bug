plain
[00:54:16] ....................................................................................................
[00:54:19] ...............................................................................................i....
[00:54:21] ....................................................................................................
[00:54:24] ....................................................................................................
[00:54:26] ............................................iiiiiiiii...............................................
[00:54:32] ....................................................................................................
[00:54:35] ....................................................................................................
[00:54:38] .......................i............................................................................
[00:54:41] ..........................i...............................................i.i..ii...................
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:19] 
[01:02:19] running 96 tests
[01:04:12] ............................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
a50d kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.000000] Policy zone: Normal
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.000000] PID hash table entries: 40963a50d kernel: [    0.354907] Initializing cgroup subsys io
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.355488] Initializing cgroup subsys memory
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.356128] Initializing cgroup subsys devices
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.356955] Initializing cgroup subsys freezer
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.357581] Initializing cgroup subsys net_cls
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.358233] Initializing cgroup subsys perf_event
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.359090] Initializing cgroup subsys net_prio
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.359716] Initializing cgroup subsys hugetlb
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.360462] Initializing cgroup subsys pids
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.361252] CPU: Physical Processor ID: 0
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.361825] CPU: Processor Core ID: 0
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.363233] mce: CPU supports 32 MCE banks
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.364023] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.364936] Last level dTLB entries: 4KB f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.690705] PCI host bridge to bus 0000:00
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.691376] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.692337] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.693364] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.694573] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.695797] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.697334] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.697768] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.711581] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.726288] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.727793] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    0.732057] pci 0000:  9.668045] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    9.719903] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    9.783255] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [    9.956890] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [   10.240092] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [   10.317379] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [   10.390958] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [   10.426576] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d kernel: [   11.402321] init: failsafe main process (1095) killed by TERM signal
Aug 15 22:08:55 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 22:08:56 travis-job-dbc74f20-053d- plymouth-upstart-bridge respawning too fast, stopped
Aug 15 23:14:59 travis-job-dbc74f20-053d-4097-b2f4-0a04b5d3a50d rsyslogd: [origin software="rsyslogd" swVersion="7.4.4" x-pid="1191" x-info="http://www.rsyslog.com"] exiting on signal 15.
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:015895c8
