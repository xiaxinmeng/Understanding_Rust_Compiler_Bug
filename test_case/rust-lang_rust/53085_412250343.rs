plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Retrying (Retry(total=4, connect=None, read=None, redirect=None)) after connection broken by 'ReadTimeoutError("HTTPSConnectionPool(host='pypi.org', port=443): Read timed out. (read timeout=15)",)': /simple/awscli/
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/9c/61/25f6aa281601d5ba73e1d111bc5b2210f6cd2d2c12ba98b9b74e1e2fe148/awscli-1.15.76-py2.py3-none-any.whl (1.3MB)
---
[01:47:41] fatal: Could not parse object '57250b2b8fa63934f80e5376a29f7dcb3f759ad6'.
[01:47:47] From https://github.com/servo/webrender
[01:47:47]  * branch            master     -> FETCH_HEAD
[01:47:47] fatal: Could not parse object '57250b2b8fa63934f80e5376a29f7dcb3f759ad6'.
[01:51:21] fatal: unable to access 'https://github.com/servo/webrender/': Failed to connect to github.com port 443: Connection timed out
[01:51:21] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:51:21] 
[01:51:21] 
[01:51:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:51:21] expected success, got: exit code: 101
[01:51:21] expected success, got: exit code: 101
[01:51:21] 
[01:51:21] 
[01:51:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:51:21] Build completed unsuccessfully in 0:36:33
[01:51:21] Makefile:60: recipe for target 'check-aux' failed
[01:51:21] make: *** [check-aux] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cbe818c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:2def2f58
$ sudo tail -n 500 /var/log/syslog
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Using GB pages for direct mapping
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] kvm-clock: using sched offset of 1587611943 cycles
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Zone ranges:
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   Device   empty
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Movable zone start for each node
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Early memory node ranges
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Policy zone: Normal
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Hierarchical RCU implementation.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] console [ttyS0] enabled
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.599241] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.605149] pid_max: default: 32768 minimum: 301
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.609002] ACPI: Core revision 20150930
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.617333] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.621590] Security Framework initialized
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.623905] Yama: becoming mindful.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.625993] AppArmor: AppArmor disabled by boot time parameter
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.631307] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.643217] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.651219] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.656163] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.660932] Initializing cgroup subsys io
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.663541] Initializing cgroup subsys memory
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.665963] Initializing cgroup subsys devices
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.668428] Initializing cgroup subsys freezer
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.670868] Initializing cgroup subsys net_cls
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.673512] Initializing cgroup subsys perf_event
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.675680] Initializing cgroup subsys net_prio
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.678513] Initializing cgroup subsys hugetlb
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.681202] Initializing cgroup subsys pids
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.683604] CPU: Physical Processor ID: 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.686235] CPU: Processor Core ID: 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.688863] mce: CPU supports 32 MCE banks
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.690736] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.693700] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.699432] Freeing SMP alternatives memory: 32K
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.710973] ftrace: allocating 32185 entries in 126 pages
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.767844] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.773042] smpboot: Max logical packages: 2
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.775894] x2apic enabled
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.778546] Switched APIC routing to physical x2apic.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.785441] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.896846] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.902531] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.908137] x86: Booting SMP configuration:
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.910255] .... node  #0, CPUs:      #1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.912566] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.918285]  #2
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.919509] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.925336]  #3
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.926530] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.932079] x86: Booted up 1 node, 4 CPUs
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.934039] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.939422] devtmpfs: initialized
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.945010] evm: security.selinux
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.947042] evm: security.SMACK64
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.949331] evm: security.SMACK64EXEC
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.951508] evm: security.SMACK64TRANSMUTE
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.953570] evm: security.SMACK64MMAP
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.955768] evm: security.ima
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.964812] evm: security.capability
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.967507] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.973937] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.978619] pinctrl core: initialized pinctrl subsystem
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.982152] RTC time:  2:42:19, date: 08/11/18
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    0.986870] NET: Registered protocol family 16
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.000954] cpuidle: using governor ladder
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.012924] cpuidle: using governor menu
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.014898] PCCT header not found.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.016892] ACPI: bus type PCI registered
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.018999] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.022926] PCI: Using configuration type 1 for base access
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.038649] ACPI: Added _OSI(Module Device)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.041288] ACPI: Added _OSI(Processor Device)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.043713] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.046963] ACPI: Added _OSI(Processor Aggregator Device)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.052706] ACPI: Executed 2 blocks of module-level executable AML code
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.079899] ACPI: Interpreter enabled
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.082007] ACPI: (supports S0 S3 S4 S5)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.085123] ACPI: Using IOAPIC for interrupt routing
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.087545] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.121722] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.125075] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.128628] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.132382] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.138808] PCI host bridge to bus 0000:00
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.141239] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.145591] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.149525] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.154243] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.159101] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.164310] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.164755] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.197288] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.224278] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.231466] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.247994] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.260638] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.294586] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.304995] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.312719] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.332227] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.338356] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.343300] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.348013] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.353126] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.375617] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.379313] vgaarb: loaded
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.380932] SCSI subsystem initialized
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.383117] libata version 3.00 loaded.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.383140] ACPI: bus type USB registered
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.385293] usbcore: registered new interface driver usbfs
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.388555] usbcore: registered new interface driver hub
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.391330] usbcore: registered new device driver usb
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.393981] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.397341] dmi: Firmware registration failed.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.400411] PCI: Using ACPI for IRQ routing
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.402977] PCI: pci_cache_line_size set to 64 bytes
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.403075] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.403078] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.403232] NetLabel: Initializing
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.405321] NetLabel:  domain hash size = 128
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.407518] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.410274] NetLabel:  unlabeled traffic allowed by default
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.413612] amd_nb: Cannot enumerate AMD northbridges
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.416666] clocksource: Switched to clocksource kvm-clock
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.426337] pnp: PnP ACPI init
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.428099] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.428206] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.428257] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.428310] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.428354] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.428396] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.428440] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.428608] pnp: PnP ACPI: found 7 devices
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.439579] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.444917] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.444920] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.444922] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.444924] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.444958] NET: Registered protocol family 2
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.447629] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.451659] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.456040] TCP: Hash tables configured (established 131072 bind 65536)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.459987] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.463327] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.467565] NET: Registered protocol family 1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.470005] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.473158] PCI: CLS 0 bytes, default 64
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    1.473998] Unpacking initramfs...
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.492515] Freeing initrd memory: 21432K
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.494994] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.498505] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.504557] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.509705] hw unit of domain pp0-core 2^-0 Joules
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.512240] hw unit of domain package 2^-0 Joules
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.514996] hw unit of domain dram 2^-0 Joules
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.517597] Scanning for low memory corruption every 60 seconds
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.521528] audit: initializing netlink subsys (disabled)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.524960] audit: type=2000 audit(1533955341.930:1): initialized
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.528417] Initialise system trusted keyring
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.531341] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.534506] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.539209] zbud: loaded
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.541362] VFS: Disk quotas dquot_6.6.0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.543828] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.548101] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.551642] fuse init (API version 7.23)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.553808] Key type big_key registered
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.555536] Allocating IMA MOK and blacklist keyrings.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.562162] Key type asymmetric registered
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.565077] Asymmetric key parser 'x509' registered
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.568441] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.572377] io scheduler noop registered
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.574081] io scheduler deadline registered (default)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.577086] io scheduler cfq registered
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.579216] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.582043] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.585035] intel_idle: does not run on family 6 model 45
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.585145] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.588204] ACPI: Power Button [PWRF]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.589957] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.593443] ACPI: Sleep Button [SLPF]
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.595826] GHES: HEST is not enabled!
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.600156] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.603698] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.612831] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.615463] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.625394] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.650100] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.676680] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.702607] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.727919] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.743894] Linux agpgart interface v0.103
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.756703] loop: module loaded
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.761946] libphy: Fixed MDIO Bus: probed
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.766010] tun: Universal TUN/TAP device driver, 1.6
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.770449] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.840953] PPP generic driver version 2.4.2
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.846399] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.854504] ehci-pci: EHCI PCI platform driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.859700] ehci-platform: EHCI generic platform driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.864638] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.872696] ohci-pci: OHCI PCI platform driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.880029] ohci-platform: OHCI generic platform driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.885484] uhci_hcd: USB Universal Host Controller Interface driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.891154] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.901148] i8042: Warning: Keylock active
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.906403] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.911558] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.917525] mousedev: PS/2 mouse device common for all mice
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.925973] rtc_cmos 00:00: RTC can wake from S4
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.932951] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.940258] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.948157] i2c /dev entries driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.951658] device-mapper: uevent: version 1.0.3
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.957067] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.967578] ledtrig-cpu: registered to indicate activity on CPUs
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.978816] NET: Registered protocol family 10
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.983805] NET: Registered protocol family 17
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.987312] Key type dns_resolver registered
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.990488] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    3.996569] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.005038] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.009737] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.014203] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.023332] registered taskstats version 1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.027521] Loading compiled-in X.509 certificates
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.034530] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.043397] zswap: loaded using pool lzo/zbud
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.051736] Key type trusted registered
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.064710] Key type encrypted registered
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.069543] ima: No TPM chip found, activating TPM-bypass!
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.079551] evm: HMAC attrs: 0x1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.083795]   Magic number: 14:730:712
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.087581] acpi LNXCPU:49: hash matches
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.091844] acpi LNXCPU:1c: hash matches
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.096623] rtc_cmos 00:00: setting system clock to 2018-08-11 02:42:23 UTC (1533955343)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.104086] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.110754] EDD information not available.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.115314] PM: Hibernation image not present or could not be loaded.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.117008] Freeing unused kernel memory: 1496K
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.121029] Write protecting the kernel read-only data: 14336k
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.125675] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.139394] Freeing unused kernel memory: 1956K
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.144875] Freeing unused kernel memory: 92K
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.172580] systemd-udevd[119]: starting version 204
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.262641] scsi host0: Virtio SCSI HBA
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.282542] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.291997] AVX version of gcm_enc/dec engaged.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.295890] AES CTR mode by8 optimization enabled
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.381377] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.381624] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.381625] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.383057] sd 0:0:1:0: [sda] Write Protect is off
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.383060] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.383444] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.385935]  sda: sda1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.388194] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.516847] tsc: Refined TSC clocksource calibration: 2600.003 MHz
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    4.525136] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ef07ae, max_idle_ns: 440795268508 ns
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    5.106110] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    7.237094] floppy0: no floppy controllers found
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.396688] raid6: sse2x1   gen()  8893 MB/s
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.464686] raid6: sse2x1   xor()  6716 MB/s
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.532684] raid6: sse2x2   gen() 11021 MB/s
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.600684] raid6: sse2x2   xor()  7348 MB/s
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.668678] raid6: sse2x4   gen() 12806 MB/s
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.736682] raid6: sse2x4   xor()  8974 MB/s
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.737321] raid6: using algorithm sse2x4 gen() 12806 MB/s
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.738101] raid6: .... xor() 8974 MB/s, rmw enabled
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.738822] raid6: using ssse3x2 recovery algorithm
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.740786] xor: automatically using best checksumming function:
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.780679]    avx       : 27667.000 MB/sec
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.794671] Btrfs loaded
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.837552] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.838542] EXT4-fs (sda1): write access will be enabled during recovery
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.892463] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.897842] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.898591] EXT4-fs (sda1): recovery complete
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    8.903287] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    9.080418] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    9.168715] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    9.218277] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    9.379693] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    9.851681] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [    9.971118] systemd-udevd[703]: starting version 204
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.055066] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.101908] intel_rapl: no valid rapl domains found in package 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.163263] ppdev: user-space parallel port driver
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.238973] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.284926] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.340376] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.502107] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.768768] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.833035] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.899883] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   10.940625] EXT4-fs (sda1): resized filesystem to 7864064
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   11.215981] init: failsafe main process (1097) killed by TERM signal
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Running set_multiqueue.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Set channels for eth0 to 4.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 11 02:42:30 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-clock-skew: INFO Clock drift token has changed: 0.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Starting Google Accounts daemon.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Creating a new user account for me.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Created user account me.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Creating a new user account for bogdana.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   11.917833] random: nonblocking pool is initialized
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Created user account bogdana.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Creating a new user account for aj.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Created user account aj.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Creating a new user account for asari.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Created user account asari.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Removing user packer.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e google-accounts: INFO Removing user packer.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e cron[1435]: (CRON) INFO (pidfile fd = 3)
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e cron[1474]: (CRON) STARTUP (fork ok)
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e cron[1474]: (CRON) INFO (Running @reboot jobs)
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e acpid: starting up with netlink and the input layer
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e acpid: 1 rule loaded
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e acpid: waiting for events: event logging is off
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e haveged: haveged starting up
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e pollinate: To re-seed this system again, use the -r|--reseed option
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   12.372002] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   12.383569] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   12.569316] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   12.572086] Bridge firewalling registered
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   12.581405] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   12.642580] Initializing XFRM netlink socket
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   12.650046] Netfilter messages via NETLINK v0.30.
Aug 11 02:42:31 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   12.652718] ctnetlink v0.93: registering with nfnetlink.
Aug 11 02:42:32 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   13.116981] floppy0: no floppy controllers found
Aug 11 02:42:54 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpdate[1776]: adjust time server 169.254.169.254 offset 0.006311 sec
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1809]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: proto: precision = 0.103 usec
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: Listen normally on 3 eth0 10.20.0.244 UDP 123
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: peers refreshed
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: Listening on routing socket on fd #21 for interface updates
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [   42.573368] init: plymouth-upstart-bridge main process ended, respawning
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e startup-script: INFO Found startup-script in metadata.
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e startup-script: INFO startup-script: job 1 at Sat Aug 11 05:53:00 2018
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e startup-script: INFO startup-script: Return code 0.
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e startup-script: INFO startup-script: Return code 0.
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e startup-script: INFO Finished running startup scripts.
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ec2: 
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ec2: #############################################################
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ec2: 1024 73:f1:0e:83:48:2d:82:5d:94:3f:95:95:8c:da:db:9b  root@travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e (DSA)
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ec2: 256 3c:e2:7d:f5:8c:de:f8:c8:42:15:b8:46:8d:1d:ca:74  root@travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e (ECDSA)
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ec2: 256 b1:8c:a8:46:47:77:b2:e0:e0:90:65:7b:04:db:8c:6b  root@travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e (ED25519)
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ec2: 2048 69:08:51:2b:b4:22:e3:e8:4f:d4:6d:d3:8f:1f:1a:2a  root@travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e (RSA)
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 11 02:43:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ec2: #############################################################
Aug 11 02:44:05 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  106.078919] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.199473] device veth2900018 entered promiscuous mode
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.199584] docker0: port 1(veth2900018) entered forwarding state
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.199593] docker0: port 1(veth2900018) entered forwarding state
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.200184] docker0: port 1(veth2900018) entered disabled state
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.327227] cgroup: docker-runc (4827) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.327230] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.410613] eth0: renamed from vethaaca466
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.451418] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.452801] docker0: port 1(veth2900018) entered forwarding state
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.452834] docker0: port 1(veth2900018) entered forwarding state
Aug 11 02:53:11 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  652.452864] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 11 02:53:14 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 11 02:53:14 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: Listen normally on 6 docker0 fe80::42:48ff:fe7a:ed01 UDP 123
Aug 11 02:53:14 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 11 02:53:14 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: peers refreshed
Aug 11 02:53:14 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e ntpd[1810]: new interface(s) found: waking up resolver
Aug 11 02:53:26 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [  667.470190] docker0: port 1(veth2900018) entered forwarding state
Aug 11 03:17:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e CRON[12839]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 11 04:04:24 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [ 4925.621107] traps: a[9259] trap invalid opcode ip:55cba1786bbb sp:7ffd2f3d08c0 error:0 in a[55cba1783000+6000]
Aug 11 04:04:45 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [ 4946.400618] traps: a[12084] trap invalid opcode ip:7f01621475a1 sp:7fff4ea42ec0 error:0 in libstd-41f43a30bc296e4f.so[7f01620ea000+166000]
Aug 11 04:04:45 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [ 4946.439080] traps: a[12085] trap invalid opcode ip:7f1bb97585a1 sp:7ffcba290d70 error:0 in libstd-41f43a30bc296e4f.so[7f1bb96fb000+166000]
Aug 11 04:06:43 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [ 5064.164799] traps: a[26919] trap invalid opcode ip:55e53c77ce29 sp:7ffdddf54e80 error:0 in a[55e53c77a000+4000]
Aug 11 04:10:27 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [ 5289.044207] a[22942]: segfault at 0 ip 000055de2c7953ef sp 00007ffc116cc170 error 6 in a[55de2c792000+5000]
Aug 11 04:10:41 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [ 5302.267209] a[23699]: segfault at 1 ip 000056088b676bed sp 00007fff7b89a2b0 error 6 in a[56088b674000+4000]
Aug 11 04:10:47 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e kernel: [ 5308.468546] traps: a[24140] trap invalid opcode ip:563cd24184bc sp:7ffcb590d490 error:0 in a[563cd2415000+7000]
Aug 11 04:17:01 travis-job-ee3d10e3-0628-4058-9fdf-a767b552f61e CRON[9630]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:0a51f7e8:start=1533962392797221134,finish=1533962392804824583,duration=7603449
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:317fbdc5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00d6f920
travis_time:start:00d6f920
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:16e87c60
$ dmesg | grep -i kill
