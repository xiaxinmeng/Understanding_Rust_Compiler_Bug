plain
[00:52:06] ....................................................................................................
[00:52:10] ...............................................................................................i....
[00:52:13] ....................................................................................................
[00:52:16] ....................................................................................................
[00:52:18] ............................................iiiiiiiii...............................................
[00:52:24] ....................................................................................................
[00:52:28] ....................................................................................................
[00:52:30] .......................i............................................................................
[00:52:33] ..........................i...............................................i.i..ii...................
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:20] 
[01:05:20] running 255 tests
[01:05:50] ...F.....F............i......................................F......................................
[01:06:16] ......................i.....F..............F........................................................
/ @has - '//*[@class="docblock"]' 'FOO_NO_DEFAULT: bool = false'
[01:06:24] 43: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]      // @has - '//*[@class="docblock"]' 'BAR: usize = 3'
[01:06:24] 52: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]      // @has - '//*[@class="docblock"]' "BAZ: Baz<'static, u8, u32> = Baz(321, &[1, 2, 3])"
[01:06:24] 61: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]      // @has - '//*[@class="docblock"]' "F: fn(_: &(ToString + 'static)) = f"
[01:06:24] 84: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]      // @has - '//*[@class="docblock"]' "QUX_DEFAULT0: u16 = 1"
[01:06:24] 89: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]      // @has - '//*[@class="docblock"]' "QUX_DEFAULT1: i16 = 2"
[01:06:24] 94: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]      // @has - '//*[@class="docblock"]' "QUX_DEFAULT2: u32 = 3"
[01:06:24] 103: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]      // @has - '//*[@class="docblock"]' "QUX0: u8 = 4"
[01:06:24] 108: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]      // @has - '//*[@class="docblock"]' "QUX1: i8 = 5"
[01:06:24] 113: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]      // @has - '//*[@class="docblock"]' "QUX_DEFAULT0: u16 = 6"
[01:06:24] 117: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]      // @has - '//*[@class="docblock"]' "QUX_DEFAULT1: i16 = 7"
[01:06:24] 122: @has check failed
[ PATTERN` did not match
[01:06:24]          // @has - '//*[@class="docblock"]' 'C: (i32, i32) = (4, 4)'
[01:06:24] 59: @has check failed
[01:06:24]  `XPATH PATTERN` did not match
[01:06:24]          // @has - '//*[@class="docblock"]' 'D: i32 = 4 / 4'
[01:06:24] Encountered 5 errors
[01:06:24] 
[01:06:24] ------------------------------------------
[01:06:24] 
---
[01:06:24] 
[01:06:24] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:06:24] 
[01:06:24] 
[01:06:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:24] 
[01:06:24] 
[01:06:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:24] Build completed unsuccessfully in 0:18:26
[01:06:24] Build completed unsuccessfully in 0:18:26
[01:06:24] make: *** [check] Error 1
[01:06:24] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01b81585
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:1b198474
$ sudo tail -n 500 /var/log/syslog
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] SRAT: Node 0 PXM 652 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] fanout_leaf=64, nr_cpu_ids=4
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] console [ttyS0] enabled
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.729055] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.736454] pid_max: default: 32768 minimum: 301
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.740775] ACPI: Core revision 20150930
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.749231] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.753356] Security Framework initialized
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.756459] Yama: becoming mindful.
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.760101] AppArmor: AppArmor disabled by boot time parameter
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.766051] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: b3-8acb-d46b3b883652 kernel: [    0.838458] CPU: Processor Core ID: 0
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.841230] mce: CPU supports 32 MCE banks
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.843795] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.849108] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.856062] Freeing SMP alternatives memory: 32K
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.869379] ftrace: allocating 32185 entries in 126 pages
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.926592] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.931553] smpboot: Max logical packages: 2
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.934945] x2apic enabled
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.937976] Switched APIC routing to physical x2apic.
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    0.945359] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.057560] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.064955] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.072701] x86: Booting SMP configuration:
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.076361] .... node  #0, CPUs:      #1
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.079484] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.087258]  #2
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.088711] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.095362]  #3
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.096487] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.103061] x86: Booted up 1 node, 4 CPUs
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.106093] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.113568] devtmpfs: initialized
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.120196] evm: security.selinux
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.122390] evm: security.SMACK64
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.124751] evm: security.SMACK64EXEC
Aug 15 22:47:44 travis-job-120: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.620746] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.620945] NetLabel: Initializing
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.624240] NetLabel:  domain hash size = 128
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.627246] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.631457] NetLabel:  unlabeled traffic allowed by default
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.635179] amd_nb: Cannot enumerate AMD northbridges
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.639149] clocksource: Switched to clocksource kvm-clock
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.650717] pnp: PnP ACPI init
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.652410] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.652489] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.652539] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.652596] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.652676] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.652727] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.652772] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.652952] pnp: PnP ACPI: found 7 devices
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.663220] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.670627] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.670630] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.670632] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.670634] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.670707] NET: Registered protocol family 2
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    1.674300] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46nel: [    4.210016] ohci-pci: OHCI PCI platform driver
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.213293] ohci-platform: OHCI generic platform driver
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.218040] uhci_hcd: USB Universal Host Controller Interface driver
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.223598] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.230475] i8042: Warning: Keylock active
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.233958] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.237777] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.242482] mousedev: PS/2 mouse device common for all mice
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.246938] rtc_cmos 00:00: RTC can wake from S4
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.250403] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.254056] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.258718] i2c /dev entries driver
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.261439] device-mapper: uevent: version 1.0.3
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.265167] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.270296] ledtrig-cpu: registered to indicate activity on CPUs
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.275907] NET: Registered protocol family 10
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.280049] NET: Registered protocol family 17
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.283211] Key type dns_resolver registered
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.286609] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.290945] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.295127] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.299305] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.303740] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.311228] registered taskstats version 1
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.314034] Loading compiled-in X.509 certificates
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.320352] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.328385] zswap: loaded using pool lzo/zbud
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.334904] Key type trusted registered
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.343398] Key type encrypted registered
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.346954] ima: No TPM chip found, activating TPM-bypass!
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.352743] evm: HMAC attrs: 0x1
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.355718]   Magic number: 14:709:805
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.360539] rtc_cmos 00:00: setting system clock to 2018-08-15 22:47:36 UTC (1534373256)
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.369372] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.373679] EDD information not available.
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.377163] PM: Hibernation image not present or could not be loaded.
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    4.378766] Freeing unused kernel memory: 1496K
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b8  8.847300] raid6: sse2x2   xor()  6982 MB/s
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    8.915386] raid6: sse2x4   gen() 12575 MB/s
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    8.983223] raid6: sse2x4   xor()  8774 MB/s
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    8.985739] raid6: using algorithm sse2x4 gen() 12575 MB/s
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    8.989371] raid6: .... xor() 8774 MB/s, rmw enabled
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    8.992114] raid6: using ssse3x2 recovery algorithm
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    8.997961] xor: automatically using best checksumming function:
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    9.039251]    avx       : 26694.000 MB/sec
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    9.058237] Btrfs loaded
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    9.148870] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    9.153937] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    9.256361] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [    9.269232] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8ac_affinity_list to 2 for device virtio1.
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 22:47:44 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 22:47:45 travis-job-1f1d3884-9831-4fb3-884-9831-4fb3-8acb-d46b3b883652 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 22:47:49 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 22:47:49 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 cron[1659]: (CRON) STARTUP (fork ok)
Aug 15 22:47:49 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 cron[1659]: (CRON) INFO (Running @reboot jobs)
Aug 15 22:47:49 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 acpid: starting up with netlink and the input layer
Aug 15 22:47:49 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 acpid: 1 rule loaded
Aug 15 22:47:49 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 acpid: waiting for events: event logging is off
Aug 15 22:47:49 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 haveged: haveged starting up
Aug 15 22:47:49 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [   17.309769] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ntpd[1765]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ntpd[1766]: proto: precision = 0.157 usec
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ntpd[1766]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ntpd[1766]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ntpd[1766]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 9831-4fb3-8acb-d46b3b883652 ec2: 
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ec2: #############################################################
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ec2: 1024 ae:0b:31:2d:63:f2:58:f1:2a:20:52:4d:fc:c2:7f:4b  root@travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 (DSA)
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ec2: 256 ef:d0:17:9f:12:7f:a9:8b:2c:d9:f5:21:08:6d:3f:88  root@travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 (ECDSA)
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ec2: 256 9e:32:a0:bb:01:39:08:ac:cc:ef:ac:6a:d5:ba:c4:8d  root@travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 (ED25519)
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ec2: 2048 4d:a5:59:f5:28:50:96:8f:61:b0:b9:84:94:66:a0:c5  root@travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 (RSA)
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 15 22:47:54 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ec2: #############################################################
Aug 15 22:48:01 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ntpdate[1845]: the NTP socket is in use, exiting
Aug 15 22:49:24 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  112.252222] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 15 22:51:08 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  216.808089] device veth1732d60 entered promiscuous mode
Aug 15 22:51:08 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  216.808167] docker0: port 1(veth1732d60) entered forwarding state
Aug 15 22:51:08 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  216.808178] docker0: port 1(veth1732d60) entered forwarding state
Aug 15 22:51:08 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  216.808502] docker0: port 1(veth1732d60) entered disabled state
Aug 15 22:51:09 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  216.909630] cgroup: docker-runc (4867) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 15 22:51:09 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  216.909633] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 15 22:51:09 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  216.997269] eth0: renamed from veth108b643
Aug 15 22:51:09 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  217.043010] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 15 22:51:09 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  217.044452] docker0: port 1(veth1732d60) entered forwarding state
Aug 15 22:51:09 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  217.044481] docker0: port 1(veth1732d60) entered forwarding state
Aug 15 22:51:09 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 kernel: [  217.044512] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 15 22:51:12 travis-job-1f1d3884-9831-4fb3-8acb-d46b3b883652 ntpd[1766]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 15 22:51:12 travis5158716 .
2819072 ./obj/build
2210172 ./obj/build/x86_64-unknown-linux-gnu
1172028 ./.git
1058064 ./src
