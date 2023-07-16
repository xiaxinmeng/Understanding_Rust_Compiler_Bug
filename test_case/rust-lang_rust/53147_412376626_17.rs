\n# struct TheDarkKnight;\n# im:"For more information about an error, try `rustc --explain E0507`.\n"}
[00:49:21] ------------------------------------------
[00:49:21] 
[00:49:21] thread '[ui] ui/nll/move-errors.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:49:21] test result: FAILED. 2243 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[00:49:21] 
[00:49:21] 
[00:49:21] 
[00:49:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:21] 
[00:49:21] 
[00:49:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:21] Build completed unsuccessfully in 0:02:17
[00:49:21] Build completed unsuccessfully in 0:02:17
[00:49:21] make: *** [check] Error 1
[00:49:21] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0404cfb8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:001973be
$ sudo tail -n 500 /var/log/syslog
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 12 21:44:38 travis-job-556f2dba-8c80avis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-90 kernel: [    0.664204] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.668773] pid_max: default: 32768 minimum: 301
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.671520] ACPI: Core revision 20150930
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.679691] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.683486] Security Framework initialized
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.685846] Yama: becoming mindful.
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.688266] AppArmor: AppArmor disabled by boot time parameter
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.693155] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.705426] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.713173] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.717812] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    0.722831] Initializing cgroup subsys io
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-aob-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.646894] Asymmetric key parser 'x509' registered
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.649865] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.653887] io scheduler noop registered
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.656445] io scheduler deadline registered (default)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.659296] io scheduler cfq registered
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.661124] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.664355] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.669172] intel_idle: does not run on family 6 model 45
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.669289] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.673927] ACPI: Power Button [PWRF]
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.676179] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.680100] ACPI: Sleep Button [SLPF]
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kern kernel: [    3.920004] i8042: Warning: Keylock active
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.924341] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.927704] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.930603] mousedev: PS/2 mouse device common for all mice
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.935013] rtc_cmos 00:00: RTC can wake from S4
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.938694] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.942224] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.945032] i2c /dev entries driver
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.946793] device-mapper: uevent: version 1.0.3
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.949333] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.953370] ledtrig-cpu: registered to indicate activity on CPUs
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.957561] NET: Registered protocol family 10
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    3.960177] NET: Registered protocol family 17
vis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.015683] ima: No TPM chip found, activating TPM-bypass!
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.018722] evm: HMAC attrs: 0x1
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.021440]   Magic number: 14:825:752
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.023537] tty ttyS3: hash matches
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.025237] acpi LNXCPU:77: hash matches
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.027033] acpi LNXCPU:4a: hash matches
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.029235] rtc_cmos 00:00: setting system clock to 2018-08-12 21:44:30 UTC (1534110270)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.034524] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.037714] EDD information not available.
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.039716] PM: Hibernation image not present or could not be loaded.
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.041593] Freeing unused kernel memory: 1496K
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.043720] Write protecting the kernel read-only data: 14336k
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    4.048354] Freeing unused kernel memory: 1956K
Aug 12 21822-a75ca29a2190 kernel: [    8.637211] raid6: sse2x4   xor()  8683 MB/s
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.639210] raid6: using algorithm sse2x4 gen() 12427 MB/s
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.642329] raid6: .... xor() 8683 MB/s, rmw enabled
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.644611] raid6: using ssse3x2 recovery algorithm
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.648711] xor: automatically using best checksumming function:
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.689177]    avx       : 27303.000 MB/sec
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.703976] Btrfs loaded
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.747977] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.749083] EXT4-fs (sda1): write access will be enabled during recovery
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.832842] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.839678] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.840469] EXT4-fs (sda1): recovery complete
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    8.845651] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    9.072722] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    9.202194] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    9.261369] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [    9.499499] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   10.152458] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   10.308463] systemd-udevd[701]: starting version 204
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   10.441686] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   10.518893] intel_rapl: no valid rapl domains found in package 0
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   10.575215] ppdev: user-space parallel port driver
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   10.695048] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   10.754078] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   10.830273] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   10.999943] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   11.289080] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   11.376457] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   11.463010] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   11.516950] EXT4-fs (sda1): resized filesystem to 7864064
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   11.850982] init: failsafe main process (1092) killed by TERM signal
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 instance-setup: INFO Running set_multiqueue.
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 instance-setup: INFO Set channels for eth0 to 4.
Aug 12 21:44:38 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for a-8c80-455f-9822-a75ca29a2190 google-accounts: INFO Created user account solarce.
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 google-accounts: INFO Creating a new user account for asari.
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 google-accounts: INFO Created user account asari.
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 google-accounts: INFO Creating a new user account for bogdana.
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 google-accounts: INFO Created user account bogdana.
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 google-accounts: INFO Creating a new user account for konstantin.
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   13.255020] random: nonblocking pool is initialized
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 google-accounts: INFO Created user account konstantin.
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 google-accounts: INFO Creating a new user account for carmen.
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   13.322801] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   13.328453] Bridge firewalling registered
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 kernel: [   13.341570] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 12 21:44:40 travis-job-556f2dba-8c80-455f-9822-a75ca29a2190 google-accounts: INFO Created user a
