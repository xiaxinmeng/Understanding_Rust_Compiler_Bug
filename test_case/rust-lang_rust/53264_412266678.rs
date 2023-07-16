plain
[00:10:34]    Compiling tidy v0.1.0 (file:///checkout/src/tools/tidy)
[00:10:44] some tidy checks failed
[00:10:44] 
[00:10:44] 
[00:10:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:10:44] 
[00:10:44] 
[00:10:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:10:44] Build completed unsuccessfully in 0:00:55
[00:10:44] Build completed unsuccessfully in 0:00:55
[00:10:44] Makefile:79: recipe for target 'tidy' failed
[00:10:44] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:294d1b68
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:2cbbfdb6
$ sudo tail -n 500 /var/log/syslog
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000]   5 disabled
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000]   6 disabled
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000]   7 disabled
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] Using GB pages for direct mapping
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 10:19:27 travis-job-95c-a840100a69d5 kernel: [    0.699435] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.705906] Freeing SMP alternatives memory: 32K
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.717992] ftrace: allocating 32185 entries in 126 pages
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.774816] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.778919] smpboot: Max logical packages: 2
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.782100] x2apic enabled
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.784930] Switched APIC routing to physical x2apic.
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.790970] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.900628] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.906016] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.912132] x86: Booting SMP configuration:
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    0.914788] .... node  #0, CPUs:      #1
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel:on space under this bridge.
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.130448] PCI host bridge to bus 0000:00
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.132784] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.137089] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.141269] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.145036] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.148849] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.151803] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.152250] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.182906] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.213362] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    1.217291] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 11 10:19:27 travis-job-95c574d2ca0-9628-a840100a69d5 kernel: [    3.859178] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.863885] i8042: Warning: Keylock active
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.866998] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.870036] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.873204] mousedev: PS/2 mouse device common for all mice
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.876779] rtc_cmos 00:00: RTC can wake from S4
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.879649] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.883979] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.886912] i2c /dev entries driver
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.888596] device-mapper: uevent: version 1.0.3
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.891461] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    3.896865] ledtrig-cpu: registered to indicate activity on CPUs
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69[    4.181280] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    4.183692]  sda: sda1
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    4.186432] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    4.542149] tsc: Refined TSC clocksource calibration: 2600.002 MHz
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    4.546347] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3e40e8d, max_idle_ns: 440795298933 ns
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    4.915163] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    7.070288] floppy0: no floppy controllers found
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.242064] raid6: sse2x1   gen()  8698 MB/s
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.310067] raid6: sse2x1   xor()  6482 MB/s
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.378069] raid6: sse2x2   gen() 10546 MB/s
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.446063] raid6: sse2x2   xor()  7094 MB/s
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.514065] raid6: sse2x4   gen() 12494 MB/s
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.582043] raid6: sse2x4   xor()  8792 MB/s
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.584658] raid6: using algorithm sse2x4 gen() 12494 MB/s
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.587676] raid6: .... xor() 8792 MB/s, rmw enabled
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.589858] raid6: using ssse3x2 recovery algorithm
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.594209] xor: automatically using best checksumming function:
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.634079]    avx       : 26882.000 MB/sec
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.651099] Btrfs loaded
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.710825] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.715782] EXT4-fs (sda1): write access will be enabled during recovery
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.822976] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.836262] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 11 10:19:27 travis-job-95c574d2-6f08-4ca0-9628-a840100a69d5 kernel: [    8.839015] EXT4-fs (sda1): recovery complete
