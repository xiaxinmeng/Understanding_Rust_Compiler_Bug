plain
[00:06:56] configure: llvm.assertions      := True
[00:06:56] configure: build.locked-deps    := True
[00:06:56] configure: llvm.ccache          := sccache
[00:06:56] configure: build.openssl-static := True
[00:06:56] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:06:56] configure: writing `config.toml` in current directory
[00:06:56] configure: 
[00:06:56] configure: run `python /checkout/x.py --help`
[00:06:56] configure: 
---
[00:08:30]     Checking panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:08:36] error[E0308]: mismatched types
[00:08:36]    --> libstd/sys/windows/fs.rs:408:45
[00:08:36]     |
[00:08:36] 408 |             let subst_ptr = path_buffer.add(subst_off);
[00:08:36] 
[00:08:37] error: aborting due to previous error
[00:08:37] 
[00:08:37] For more information about this error, try `rustc --explain E0308`.
[00:08:37] For more information about this error, try `rustc --explain E0308`.
[00:08:37] error: Could not compile `std`.
[00:08:37] 
[00:08:37] Caused by:
[00:08:37]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,metadata -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=8e91e34b7d64656a -C extra-filename=-8e91e34b7d64656a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps --target i686-pc-windows-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liballoc-3b9d32553cc83eca.rmeta --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liballoc_jemalloc-7b82ec3aa4b996b4.rmeta --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liballoc_system-53a9728ee485a57b.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libcompiler_builtins-5aaf3d6d4de03b94.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libcore-b399696d5bd47db7.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liblibc-98ca3e688c1db3c6.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libpanic_abort-65282f1ac7f26bae.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libpanic_unwind-f57f6107ac35bda7.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libunwind-fe68698e5238595a.rmeta -L native=/checkout/obj/build/i686-pc-windows-gnu/native/libbacktrace/ -L native=/checkout/obj/build/i686-pc-windows-gnu/native/libbacktrace -l static=backtrace -l static=backtrace -l advapi32 -l ws2_32 -l userenv -l shell32 -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/build/compiler_builtins-ed6515dfcadbf7bb/out -L native=/checkout/obj/build/i686-pc-windows-gnu/native/jemalloc/lib` (exit code: 1)
[00:08:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:08:37] expected success, got: exit code: 101
[00:08:37] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:08:37] travis_fold:end:stage0-std

[00:08:37] travis_time:end:stage0-std:start=1534247867678924115,finish=1534247900465455034,duration=32786530919

---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:2dbc9681
$ sudo tail -n 500 /var/log/syslog
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   1 disabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   2 disabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   3 disabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   4 disabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   5 disabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   6 disabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   7 disabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Using GB pages for direct mapping
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] kvm-clock: using sched offset of 2116799146 cycles
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Zone ranges:
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   Device   empty
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Movable zone start for each node
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Early memory node ranges
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Policy zone: Normal
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] console [ttyS0] enabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.463279] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.466144] pid_max: default: 32768 minimum: 301
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.468207] ACPI: Core revision 20150930
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.476580] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.479897] Security Framework initialized
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.481348] Yama: becoming mindful.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.483330] AppArmor: AppArmor disabled by boot time parameter
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.487150] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.499870] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.507425] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.509452] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.513624] Initializing cgroup subsys io
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.516055] Initializing cgroup subsys memory
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.518060] Initializing cgroup subsys devices
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.519756] Initializing cgroup subsys freezer
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.521630] Initializing cgroup subsys net_cls
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.523461] Initializing cgroup subsys perf_event
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.525328] Initializing cgroup subsys net_prio
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.528169] Initializing cgroup subsys hugetlb
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.530012] Initializing cgroup subsys pids
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.531694] CPU: Physical Processor ID: 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.533961] CPU: Processor Core ID: 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.536714] mce: CPU supports 32 MCE banks
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.538341] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.540989] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.546165] Freeing SMP alternatives memory: 32K
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.558382] ftrace: allocating 32185 entries in 126 pages
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.618212] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.621976] smpboot: Max logical packages: 2
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.624630] x2apic enabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.627029] Switched APIC routing to physical x2apic.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.632519] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.741874] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.746151] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.751688] x86: Booting SMP configuration:
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.753194] .... node  #0, CPUs:      #1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.754710] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.760760]  #2
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.761618] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.767692]  #3
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.768897] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.774308] x86: Booted up 1 node, 4 CPUs
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.775799] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.780278] devtmpfs: initialized
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.785833] evm: security.selinux
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.787019] evm: security.SMACK64
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.788384] evm: security.SMACK64EXEC
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.789930] evm: security.SMACK64TRANSMUTE
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.791688] evm: security.SMACK64MMAP
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.792735] evm: security.ima
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.793578] evm: security.capability
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.795124] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.798854] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.801905] pinctrl core: initialized pinctrl subsystem
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.803727] RTC time: 11:47:38, date: 08/14/18
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.806258] NET: Registered protocol family 16
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.817926] cpuidle: using governor ladder
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.829941] cpuidle: using governor menu
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.831599] PCCT header not found.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.833135] ACPI: bus type PCI registered
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.835158] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.837594] PCI: Using configuration type 1 for base access
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.851449] ACPI: Added _OSI(Module Device)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.852955] ACPI: Added _OSI(Processor Device)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.855177] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.857505] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.862680] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.888986] ACPI: Interpreter enabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.890892] ACPI: (supports S0 S3 S4 S5)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.892162] ACPI: Using IOAPIC for interrupt routing
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.894169] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.928569] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.931748] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.934301] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.937105] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.942318] PCI host bridge to bus 0000:00
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.943622] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.945572] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.948155] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.950670] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.953934] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.955984] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.956497] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    0.980574] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.005190] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.008550] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.020025] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.029451] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.052667] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.066504] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.075163] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.102808] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.107590] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.113934] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.118692] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.123967] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.147862] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.150944] vgaarb: loaded
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.152474] SCSI subsystem initialized
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.153889] libata version 3.00 loaded.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.153917] ACPI: bus type USB registered
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.155383] usbcore: registered new interface driver usbfs
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.157787] usbcore: registered new interface driver hub
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.159843] usbcore: registered new device driver usb
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.161896] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.164591] dmi: Firmware registration failed.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.166121] PCI: Using ACPI for IRQ routing
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.168069] PCI: pci_cache_line_size set to 64 bytes
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.168178] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.168180] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.168321] NetLabel: Initializing
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.169678] NetLabel:  domain hash size = 128
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.171276] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.173065] NetLabel:  unlabeled traffic allowed by default
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.175332] amd_nb: Cannot enumerate AMD northbridges
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.176988] clocksource: Switched to clocksource kvm-clock
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.186956] pnp: PnP ACPI init
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.188848] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.188915] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.188957] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.189020] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.189060] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.189100] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.189138] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.189303] pnp: PnP ACPI: found 7 devices
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.198141] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.201752] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.201755] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.201757] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.201759] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.201813] NET: Registered protocol family 2
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.204280] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.207189] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.211225] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.213671] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.216178] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.218616] NET: Registered protocol family 1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.220299] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.222193] PCI: CLS 0 bytes, default 64
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    1.223067] Unpacking initramfs...
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.398272] Freeing initrd memory: 21432K
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.399920] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.402083] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.406319] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.410292] hw unit of domain pp0-core 2^-0 Joules
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.412022] hw unit of domain package 2^-0 Joules
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.413521] hw unit of domain dram 2^-16 Joules
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.415442] Scanning for low memory corruption every 60 seconds
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.418159] audit: initializing netlink subsys (disabled)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.420670] audit: type=2000 audit(1534247261.215:1): initialized
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.423743] Initialise system trusted keyring
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.426028] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.428729] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.432061] zbud: loaded
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.433506] VFS: Disk quotas dquot_6.6.0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.434762] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.438221] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.440909] fuse init (API version 7.23)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.442569] Key type big_key registered
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.444080] Allocating IMA MOK and blacklist keyrings.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.449367] Key type asymmetric registered
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.451027] Asymmetric key parser 'x509' registered
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.453278] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.456019] io scheduler noop registered
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.457658] io scheduler deadline registered (default)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.459750] io scheduler cfq registered
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.461544] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.463907] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.466483] intel_idle: does not run on family 6 model 63
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.466613] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.468902] ACPI: Power Button [PWRF]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.470360] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.472969] ACPI: Sleep Button [SLPF]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.475110] GHES: HEST is not enabled!
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.480385] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.482712] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.492137] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.494054] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.501754] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.525630] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.550019] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.574028] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.598884] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.603081] Linux agpgart interface v0.103
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.607626] loop: module loaded
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.608940] libphy: Fixed MDIO Bus: probed
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.610213] tun: Universal TUN/TAP device driver, 1.6
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.612192] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.667273] PPP generic driver version 2.4.2
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.668851] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.670989] ehci-pci: EHCI PCI platform driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.672640] ehci-platform: EHCI generic platform driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.674178] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.675943] ohci-pci: OHCI PCI platform driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.677652] ohci-platform: OHCI generic platform driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.679183] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.681769] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.685039] i8042: Warning: Keylock active
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.687325] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.689113] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.691101] mousedev: PS/2 mouse device common for all mice
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.693964] rtc_cmos 00:00: RTC can wake from S4
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.695842] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.698582] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.701436] i2c /dev entries driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.702674] device-mapper: uevent: version 1.0.3
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.704624] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.707733] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.710978] NET: Registered protocol family 10
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.713017] NET: Registered protocol family 17
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.714567] Key type dns_resolver registered
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.716342] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.718401] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.720276] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.722261] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.724123] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.727345] registered taskstats version 1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.728663] Loading compiled-in X.509 certificates
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.731076] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.734939] zswap: loaded using pool lzo/zbud
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.739402] Key type trusted registered
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.745799] Key type encrypted registered
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.747660] ima: No TPM chip found, activating TPM-bypass!
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.750566] evm: HMAC attrs: 0x1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.752285]   Magic number: 14:465:782
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.753860] acpi LNXCPU:99: hash matches
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.755540] acpi LNXCPU:6c: hash matches
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.757857] rtc_cmos 00:00: setting system clock to 2018-08-14 11:47:41 UTC (1534247261)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.761010] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.763486] EDD information not available.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.765202] PM: Hibernation image not present or could not be loaded.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.766834] Freeing unused kernel memory: 1496K
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.768482] Write protecting the kernel read-only data: 14336k
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.772078] Freeing unused kernel memory: 1956K
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.773857] Freeing unused kernel memory: 92K
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.791595] systemd-udevd[118]: starting version 204
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.852682] scsi host0: Virtio SCSI HBA
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.858954] AVX2 version of gcm_enc/dec engaged.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.860672] AES CTR mode by8 optimization enabled
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.861130] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.897648] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.911445] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.911467] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.911469] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.919344] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.921386] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.921602] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.926637]  sda: sda1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    3.928436] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    4.413130] tsc: Refined TSC clocksource calibration: 2299.811 MHz
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    4.415660] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212682b352c, max_idle_ns: 440795264513 ns
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    4.734262] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    6.873206] floppy0: no floppy controllers found
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.029092] raid6: sse2x1   gen()  9038 MB/s
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.097035] raid6: sse2x1   xor()  6599 MB/s
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.165062] raid6: sse2x2   gen() 11241 MB/s
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.233062] raid6: sse2x2   xor()  7731 MB/s
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.301054] raid6: sse2x4   gen() 12555 MB/s
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.369038] raid6: sse2x4   xor()  8552 MB/s
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.437070] raid6: avx2x1   gen() 17120 MB/s
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.505056] raid6: avx2x2   gen() 20446 MB/s
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.573083] raid6: avx2x4   gen() 21556 MB/s
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.575019] raid6: using algorithm avx2x4 gen() 21556 MB/s
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.576925] raid6: using avx2x2 recovery algorithm
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.580047] xor: automatically using best checksumming function:
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.621030]    avx       : 21632.000 MB/sec
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.636970] Btrfs loaded
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.688705] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.691144] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.766798] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.773204] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.774023] EXT4-fs (sda1): recovery complete
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.778394] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    8.980624] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    9.105982] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    9.156474] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    9.364044] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [    9.894504] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   10.035205] systemd-udevd[701]: starting version 204
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   10.150244] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   10.202069] intel_rapl: no valid rapl domains found in package 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   10.257500] ppdev: user-space parallel port driver
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   10.352562] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   10.401390] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   10.464643] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   10.630425] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   10.878571] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   10.956108] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   11.039898] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   11.081043] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   11.508134] init: failsafe main process (1093) killed by TERM signal
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Running set_multiqueue.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Set channels for eth0 to 4.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 11:47:49 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   12.205928] random: nonblocking pool is initialized
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 google-accounts: INFO Creating a new user account for me.
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 google-accounts: INFO Created user account me.
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 google-accounts: INFO Removing user packer.
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 cron[1402]: (CRON) INFO (pidfile fd = 3)
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 cron[1442]: (CRON) STARTUP (fork ok)
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 cron[1442]: (CRON) INFO (Running @reboot jobs)
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 acpid: starting up with netlink and the input layer
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 acpid: 1 rule loaded
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 acpid: waiting for events: event logging is off
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 haveged: haveged starting up
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   12.703340] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 11:47:50 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   12.712601] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 11:47:51 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 11:47:51 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   12.905947] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 11:47:51 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   12.910069] Bridge firewalling registered
Aug 14 11:47:51 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   12.920134] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 11:47:51 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   12.991728] Initializing XFRM netlink socket
Aug 14 11:47:51 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   12.998926] Netfilter messages via NETLINK v0.30.
Aug 14 11:47:51 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   13.001725] ctnetlink v0.93: registering with nfnetlink.
Aug 14 11:47:51 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   13.201171] floppy0: no floppy controllers found
Aug 14 11:48:14 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpdate[1737]: adjust time server 169.254.169.254 offset 0.003373 sec
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1773]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: proto: precision = 0.111 usec
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: Listen normally on 3 eth0 10.20.0.95 UDP 123
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: peers refreshed
Aug 14 11:48:20 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: Listening on routing socket on fd #21 for interface updates
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [   42.904191] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 startup-script: INFO Found startup-script in metadata.
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 startup-script: INFO startup-script: job 1 at Tue Aug 14 14:58:00 2018
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 startup-script: INFO startup-script: Return code 0.
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 startup-script: INFO startup-script: Return code 0.
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 startup-script: INFO Finished running startup scripts.
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ec2: 
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ec2: #############################################################
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ec2: 1024 24:f5:ff:82:66:3c:e1:c6:40:2e:fc:0e:49:5b:6f:ef  root@travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 (DSA)
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ec2: 256 03:cc:af:b4:dd:66:26:b7:bd:e5:22:d9:5e:20:37:7e  root@travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 (ECDSA)
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ec2: 256 b4:d6:14:58:ab:e7:25:8f:82:88:05:e2:b1:92:a1:55  root@travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 (ED25519)
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ec2: 2048 1d:5b:1d:5b:d9:af:af:c5:61:c1:61:e5:40:80:5b:7c  root@travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 (RSA)
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 11:48:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ec2: #############################################################
Aug 14 11:49:42 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  124.249808] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 11:51:05 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.641923] device veth9ab1965 entered promiscuous mode
Aug 14 11:51:05 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.641993] docker0: port 1(veth9ab1965) entered forwarding state
Aug 14 11:51:05 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.642001] docker0: port 1(veth9ab1965) entered forwarding state
Aug 14 11:51:05 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.642497] docker0: port 1(veth9ab1965) entered disabled state
Aug 14 11:51:05 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.753484] cgroup: docker-runc (4767) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 11:51:05 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.753487] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 11:51:06 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.826341] eth0: renamed from veth3b187ab
Aug 14 11:51:06 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.862730] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 11:51:06 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.864004] docker0: port 1(veth9ab1965) entered forwarding state
Aug 14 11:51:06 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.864021] docker0: port 1(veth9ab1965) entered forwarding state
Aug 14 11:51:06 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  207.864044] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 11:51:08 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: Listen normally on 5 docker0 fe80::42:77ff:feda:a122 UDP 123
Aug 14 11:51:08 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 14 11:51:08 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 11:51:08 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: peers refreshed
Aug 14 11:51:08 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 ntpd[1774]: new interface(s) found: waking up resolver
Aug 14 11:51:21 travis-job-08b5c602-e709-466f-a5a8-6f54a20f84a3 kernel: [  222.908878] docker0: port 1(veth9ab1965) entered forwarding state
---
travis_time:end:04cbada9:start=1534247901130252133,finish=1534247901138539739,duration=8287606
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e527a13
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:076b4489
travis_time:start:076b4489
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:092fb5c0
$ dmesg | grep -i kill
