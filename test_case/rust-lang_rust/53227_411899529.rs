plain
[00:49:14] ....................................................................................................
[00:49:17] ....................................................................................................
[00:49:19] ....................................................................................................
[00:49:22] ....................................................................................................
[00:49:25] ...............iiiiiiiii............................................................................
[00:49:31] ....................................................................................................
[00:49:34] ....................i...............................................................................
[00:49:37] ..............................i.....................................................................
[00:49:40] ....................................................................................................
---
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:52]    Compiling libc v0.2.43
[01:03:53]    Compiling rand v0.4.2
[01:03:56]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:03:57] error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g. `MyStruct<T>`)
[01:03:57]    --> liballoc/pin.rs:137:1
[01:03:57]     |
[01:03:57] 137 | impl<T: Unpin + ?Sized> From<PinBox<T>> for Box<T> {
[01:03:57]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `T` must be used as the type parameter for some local type
[01:03:57]     |
[01:03:57]     = note: only traits defined in the current crate can be implemented for a type parameter
[01:03:57] error: aborting due to previous error
[01:03:57] 
[01:03:57] For more information about this error, try `rustc --explain E0210`.
[01:03:57] error: Could not compile `alloc`.
[01:03:57] error: Could not compile `alloc`.
[01:03:57] warning: build failed, waiting for other jobs to finish...
bd91d kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 20:08:58 travis-j42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] Policy zone: Normal
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [ o physical x2apic.
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.465455] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.571866] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.573573] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.576026] x86: Booting SMP configuration:
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.576941] .... node  #0, CPUs:      #1
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.577896] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.581314]  #2
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.581881] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.585842]  #3
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.586287] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.589873] x86: Booted up 1 node, 4 CPUs
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.590622] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  9 20:08:58 travis-job-f6 [    0.724403] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.725264] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.725671] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.742058] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.758931] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.760445] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.767254] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.772956] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.787035] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.792479] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.797604] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.812686] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 20:08:58 travis-job-f6cc424965-9685-b1e5228bd91d kernel: [    0.877083] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.877124] NET: Registered protocol family 2
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.878132] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.880365] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.881727] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.882854] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.883954] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.886122] NET: Registered protocol family 1
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.887250] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.888338] PCI: CLS 0 bytes, default 64
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    0.888405] Unpacking initramfs...
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    2.916584] Freeing initrd memory: 21432K
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel   3.141203] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.143858] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.146277] i2c /dev entries driver
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.147610] device-mapper: uevent: version 1.0.3
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.148758] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.151240] ledtrig-cpu: registered to indicate activity on CPUs
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.153298] NET: Registered protocol family 10
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.154509] NET: Registered protocol family 17
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.155653] Key type dns_resolver registered
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.157093] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.158841] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.160035] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.161110] microcode: CPU3 sig=0x206d7,6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.184997] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.186086] EDD information not available.
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.186994] PM: Hibernation image not present or could not be loaded.
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.188512] Freeing unused kernel memory: 1496K
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.189613] Write protecting the kernel read-only data: 14336k
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.191328] Freeing unused kernel memory: 1956K
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.192232] Freeing unused kernel memory: 92K
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.206742] systemd-udevd[119]: starting version 204
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.262658] scsi host0: Virtio SCSI HBA
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.268908] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.273008] AVX version of gcm_enc/dec engaged.
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.274262] AES CTR mode by8 optimization enabled
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.316621] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.316681] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.316683] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.316862] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.316865] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.316978] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.318880]  sda: sda1
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.319661] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.337941] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.925730] tsc: Refined TSC clocksource calibration: 2599.998 MHz
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    3.927349] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3a7b45f, max_idle_ns: 440795312762 ns
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    4.170691] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    9.436931] systemd-udevd[701]: starting version 204
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    9.593107] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    9.618659] intel_rapl: no valid rapl domains found in package 0
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    9.678445] ppdev: user-space parallel port driver
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    9.785872] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    9.837136] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [    9.903108] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [   10.066624] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [   10.422173] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug  9 20:08:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [   10.504255] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  9 20:08:58 t
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d cron[1499]: (CRON) INFO (Running @reboot jobs)
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d google-accounts: INFO Created user account asari.
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d google-accounts: INFO Creating a new user account for bogdana.
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d acpid: starting up with netlink and the input layer
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d acpid: 1 rule loaded
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d acpid: waiting for events: event logging is off
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d haveged: haveged starting up
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d google-accounts: INFO Created user account bogdana.
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d google-accounts: INFO Creating a new user account for konstantin.
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d google-accounts: INFO Created user account konstantin.
Aug  9 20:08:59 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d google-accounts: INFO Creating a new user acco42c1-e9d2-4965-9685-b1e5228bd91d (ED25519)
Aug  9 20:09:29 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d ec2: 2048 13:85:f0:5b:30:a4:f7:e9:46:bb:4b:3a:93:d3:8f:27  root@travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d (RSA)
Aug  9 20:09:29 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 20:09:29 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d ec2: #############################################################
Aug  9 20:09:58 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [   71.542116] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 20:11:04 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [  137.438896] device veth4ecc53b entered promiscuous mode
Aug  9 20:11:04 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [  137.438973] docker0: port 1(veth4ecc53b) entered forwarding state
Aug  9 20:11:04 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [  137.438982] docker0: port 1(veth4ecc53b) entered forwarding state
Aug  9 20:11:04 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [  137.439654] docker0: port 1(veth4ecc53b) entered disabled state
Aug  9 20:11:04 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [  137.535399] cgroup: docker-runc (4880) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 20:11:04 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [  137.535403] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 20:11:04 travis-job-f6cc42c1-e9d2-4965-9685-b1e5228bd91d kernel: [  137.594
