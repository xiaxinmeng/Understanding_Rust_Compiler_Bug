plain
[00:51:05] ....................................................................................................
[00:51:09] ...............................................................................................i....
[00:51:11] ....................................................................................................
[00:51:15] ....................................................................................................
[00:51:17] ............................................iiiiiiiii...............................................
[00:51:23] ....................................................................................................
[00:51:27] ....................................................................................................
[00:51:30] .......................i............................................................................
[00:51:33] ..........................i...............................................i.i..ii...................
---
[00:57:36] ....................................................................................................
[00:57:46] ....................................................................................................
[00:57:55] ....................................................................................................
[00:58:07] ....................................................................................................
[00:58:17] ...........................i...................................F....................................
8-9231-1795d6f7a946 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.000000] Policy zone: Normal
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.000000] Calgary: Unatries in 126 pages
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.654464] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.657985] smpboot: Max logical packages: 2
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.660442] x2apic enabled
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.663900] Switched APIC routing to physical x2apic.
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.669505] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.779384] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.783775] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.790165] x86: Booting SMP configuration:
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.793186] .... node  #0, CPUs:      #1
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.795181] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.801702]  #2
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    0.802548] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 01:44:41 travis-job-9avis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.122903] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.139766] scsi host0: Virtio SCSI HBA
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.146917] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.154072] AVX version of gcm_enc/dec engaged.
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.156690] AES CTR mode by8 optimization enabled
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.238119] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.238143] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.238145] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.246815] sd 0:0:1:0: [sda] Write Protect is off
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.249175] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.249606] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [    4.257171]  sda: sda1
Aug 15 01:44:   11.237707] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [   11.522952] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [   11.616726] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [   11.717271] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [   11.771910] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 kernel: [   12.168579] init: failsafe main process (1093) killed by TERM signal
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 instance-setup: INFO Running set_multiqueue.
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 instance-setup: INFO Set channels for eth0 to 4.
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 15 01:44:41 travis-job-939eb2f5-ba93-4ae8-9231-1795d6f7a946 instance-setup: INFO /proc/irq/26/sm
