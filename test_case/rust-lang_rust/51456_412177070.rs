plain
[00:51:04] ....................................................................................................
[00:51:07] .....................i..............................................................................
[00:51:10] ................................i...................................................................
[00:51:14] ....................................................................................................
[00:51:18] ............F.......................................................................................
[00:51:24] ..............................................i....
[00:51:24] failures:
[00:51:24] 
[00:51:24] ---- [ui] ui/rust-2018/issue-52202-use-suggestions.rs stdout ----
[00:51:24] ---- [ui] ui/rust-2018/issue-52202-use-suggestions.rs stdout ----
[00:51:24] diff of stderr:
[00:51:24] 
[00:51:24] 9    |
[00:51:24] 10 LL | use std::collections::binary_heap::Drain;
[00:51:24] 11    |
[00:51:24] - LL | use std::collections::hash_map::Drain;
[00:51:24] + LL | use std::collections::hash::map::Drain;
[00:51:24] 13    |
[00:51:24] - LL | use std::collections::hash_set::Drain;
[00:51:24] + LL | use std::collections::hash::set::Drain;
[00:51:24] - and 3 other candidates
[00:51:24] - and 3 other candidates
[00:51:24] + and 6 other candidates
[00:51:24] 18 error: aborting due to previous error
[00:51:24] 19 
[00:51:24] 
[00:51:24] 
[00:51:24] 
[00:51:24] The actual stderr differed from the expected stderr.
[00:51:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-52202-use-suggestions/issue-52202-use-suggestions.stderr
[00:51:24] To update references, rerun the tests and pass the `--bless` flag
[00:51:24]ility":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::collections::vec_deque::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::string::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::vec::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0422]: cannot find struct, variant or union type `Drain` in this scope\n  --> /checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs:21:14\n   |\nLL |     let _d = Drain {};\n   |              ^^^^^ not found in this scope\nhelp: possible candidates are found in other modules, you can import them into scope\n   |\nLL | use crate::plumbing::Drain;\n   |\nLL | use std::collse" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:24] 
[00:51:24] 
[00:51:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:24] Build completed unsuccessfully in 0:02:17
[00:51:24] Build completed unsuccessfully in 0:02:17
[00:51:24] Makefile:58: recipe for target 'check' failed
[00:51:24] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003e7390
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:13fe6b6a
$ sudo tail -n 500 /var/log/syslog
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 10 18:08:28 travis-job-dfa19e41-x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] kvm-clock: using sched offset of 1618603031 cycles
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Zone ranges:
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   Device   empty
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Movable zone start for each node
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Early memory node ranges
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 18:08:28 travis-job-dfa19e41-4c53-I (MADT) for SMP configuration information
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Policy zone: Normal
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] console [ttyS0] enabled
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.476938] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.480220] pid_max: default: 32768 minimum: 301
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1efcal 0 to logical package 0
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.628906] smpboot: Max logical packages: 2
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.631620] x2apic enabled
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.634293] Switched APIC routing to physical x2apic.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.639331] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.750638] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.754902] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.760374] x86: Booting SMP configuration:
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.762242] .... node  #0, CPUs:      #1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.764104] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.770483]  #2
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.771802] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.777565]  #3
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.778641] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.784905] x86: Booted up 1 node, 4 CPUs
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.786726] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.790323] devtmpfs: initialized
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.796175] evm: security.selinux
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.797753] evm: security.SMACK64
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.798962] evm: security.SMACK64EXEC
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.800551] evm: security.SMACK64TRANSMUTE
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.801935] evm: security.SMACK64MMAP
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.803304] evm: security.ima
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.804460] evm: security.capability
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.806247] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.810607] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.814093] pinctrl coevel executable AML code
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.901024] ACPI: Interpreter enabled
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.902838] ACPI: (supports S0 S3 S4 S5)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.904163] ACPI: Using IOAPIC for interrupt routing
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.906362] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.939445] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.941755] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.944966] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.947401] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.953311] PCI host bridge to bus 0000:00
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.955109] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    0.957558] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 18:08:28 travis-jodfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.181413] clocksource: Switched to clocksource kvm-clock
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.190913] pnp: PnP ACPI init
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.192572] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.192742] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.192791] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.192842] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.192888] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.192930] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.192969] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.193137] pnp: PnP ACPI: found 7 devices
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.203454] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.206875] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.206878] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.206880] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.206882] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.206925] NET: Registered protocol family 2
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.209020] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.213553] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.216390] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.219169] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.220935] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.224388] NET: Registered protocol family 1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    1.226150] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2 kernel: [    3.358024] Initialise system trusted keyring
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.358974] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.360605] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.362728] zbud: loaded
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.363456] VFS: Disk quotas dquot_6.6.0
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.364163] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.365655] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.367080] fuse init (API version 7.23)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.368140] Key type big_key registered
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.369013] Allocating IMA MOK and blacklist keyrings.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.370888] Key type asymmetric registered
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.371615] Asymmetric key parser 'x509' registered
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.372369] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.373700] io scheduler noop registered
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.374559] io scheduler deadline registered (default)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.375321] io scheduler cfq registered
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.376248] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.377259] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.378462] intel_idle: does not run on family 6 model 62
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.378571] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.379770] ACPI: Power Button [PWRF]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.380695] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.381824] ACPI: Sleep Button [SLPF]
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.383025] GHES: HEST is not enabled!
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.386493] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.387917] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.393052] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.394177] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.399395] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.422241] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.445917] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.469328] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.492917] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.496486] Linux agpgart interface v0.103
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.500209] loop: module loaded
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.501098] libphy: Fixed MDIO Bus: probed
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.501984] tun: Universal TUN/TAP device driver, 1.6
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.503160] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.536664] PPP generic driver version 2.4.2
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.538033] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.539544] ehci-pci: EHCI PCI platform driver
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.540601] ehci-platform: EHCI generic platform driver
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.542014] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.543566] ohci-pci: OHCI PCI platform driver
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.544605] ohci-platform: OHCI generic platform driver
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.545993] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.547821] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.550530] i8042: Warning: Keylock active
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.552664] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.553965] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.555703] mousedev: PS/2 mouse device common for all mice
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.557644] rtc_cmos 00:00: RTC can wake from S4
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.559281] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.560970] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.562447] i2c /dev entries driver
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.563259] device-mapper: uevent: version 1.0.3
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.564422] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.566310] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.568373] NET: Registered protocol family 10
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.569710] NET: Registered protocol family 17
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.570793] Key type dns_resolver registered
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.572076] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.573431] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.574751] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.576143] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.577644] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.579842] registered taskstats version 1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.580588] Loading compiled-in X.509 certificates
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.582316] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.584852] zswap: loaded using pool lzo/zbud
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.588104] Key type trusted registered
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.592359] Key type encrypted registered
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.593543] ima: No TPM chip found, activating TPM-bypass!
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.594830] evm: HMAC attrs: 0x1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.596047]   Magic number: 14:736:139
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.597100] acpi device:10: hash matches
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.598920] rtc_cmos 00:00: setting system clock to 2018-08-10 18:08:20 UTC (1533924500)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.601404] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.603077] EDD information not available.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.604359] PM: Hibernation image not present or could not be loaded.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.606318] Freeing unused kernel memory: 1496K
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.607092] Write protecting the kernel read-only data: 14336k
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.609428] Freeing unused kernel memory: 1956K
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.610620] Freeing unused kernel memory: 92K
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.625871] systemd-udevd[119]: starting version 204
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.680908] scsi host0: Virtio SCSI HBA
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    3.685548] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSg 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    4.355623] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086be2dcf, max_idle_ns: 440795324453 ns
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    4.595743] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    6.673618] floppy0: no floppy controllers found
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    7.869458] raid6: sse2x1   gen()  8709 MB/s
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    7.937453] raid6: sse2x1   xor()  6248 MB/s
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.005463] raid6: sse2x2   gen() 10742 MB/s
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.073453] raid6: sse2x2   xor()  7289 MB/s
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.141455] raid6: sse2x4   gen() 11962 MB/s
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.209461] raid6: sse2x4   xor()  8113 MB/s
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.211651] raid6: using algorithm sse2x4 gen() 11962 MB/s
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.213471] raid6: .... xor() 8113 MB/s, rmw enabled
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.215764] raid6: using ssse3x2 recovery algorithm
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.219342] xor: automatically using best checksumming function:
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.257453]    avx       : 21574.000 MB/sec
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.274764] Btrfs loaded
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.334063] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.337109] EXT4-fs (sda1): write access will be enabled during recovery
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.418315] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.433704] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.435499] EXT4-fs (sda1): recovery complete
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.444089] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.674525] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.812552] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [    8.873270] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 18:08:28c53-422a-a67b-1ef2f6685502 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 10 18:08:28 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 instance-setup: INFO /proc/irq/32/smp_affinity_list: reaO Creating a new user account for emma.
Aug 10 18:08:29 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 google-accounts: INFO Created user account emma.
Aug 10 18:08:29 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 google-accounts: INFO Creating a new user account for igor.
Aug 10 18:08:29 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 google-accounts: INFO Created user account igor.
Aug 10 18:08:29 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 10 18:08:29 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [   12.618884] random: nonblocking pool is initialized
Aug 10 18:08:29 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 google-accounts: INFO Created user account konstantinhaase.
Aug 10 18:08:30 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 18:08:30 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 google-accounts: INFO Creating a new user account for aj.
Aug 10 18:08:30 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 google-accounts: INFO Created user account aj.
Aug 10 18:08:30 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 google-accounts: INFO Creating a new user account for solarce.
Aug 10 18:08:30 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 google-accounts: INFO Created user account solarce.
Aug 10 18:08:30 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [   12.800006] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 18:08:30 travis-job-dfa19e41-4c53-422-422a-a67b-1ef2f6685502 acpid: waiting for events: event logging is off
Aug 10 18:08:30 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 haveged: haveged starting up
Aug 10 18:08:30 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [   13.546580] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 18:08:53 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpdate[1841]: adjust time server 169.254.169.254 offset 0.002264 sec
Aug 10 18:09:00 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1863]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 18:09:00 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: proto: precision = 0.123 usec
Aug 10 18:09:00 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 18:09:00 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 18:09:00 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 18:09:00 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 18:09:00 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 18:09:00 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: Listen normally on 3 eth0 10.20.1.195 UDP 123
Aug 10 18:09:00 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 18:09:00 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: peers refrb0:78:a6:4a:a6  root@travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 (ECDSA)
Aug 10 18:09:01 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ec2: 256 4a:90:39:52:44:d8:e1:37:af:b9:1e:d3:84:af:4f:30  root@travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 (ED25519)
Aug 10 18:09:01 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ec2: 2048 98:45:3f:e3:ae:23:12:96:c2:8b:68:70:ba:fe:7f:fc  root@travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 (RSA)
Aug 10 18:09:01 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 18:09:01 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ec2: #############################################################
Aug 10 18:14:38 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [  381.590801] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 18:17:01 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 CRON[4240]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 18:18:34 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [  616.830120] device veth029f69d entered promiscuous mode
Aug 10 18:18:34 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [  616.938674] cgroup: docker-runc (4890) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 18:18:34 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [  616.938676] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 18:18:34 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [  617.010245] eth0: renamed from vethb1f5b1a
Aug 10 18:18:34 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [  617.050721] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 18:18:34 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [  617.052071] docker0: port 1(veth029f69d) entered forwarding state
Aug 10 18:18:34 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [  617.052090] docker0: port 1(veth029f69d) entered forwarding state
Aug 10 18:18:34 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [  617.052112] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 18:18:37 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: Listen normally on 5 docker0 fe80::42:e3ff:fe75:152d UDP 123
Aug 10 18:18:37 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 10 18:18:37 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 18:18:37 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: peers refreshed
Aug 10 18:18:37 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 ntpd[1864]: new interface(s) found: waking up resolver
Aug 10 18:18:49 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [  632.069881] docker0: port 1(veth029f69d) entered forwarding state
Aug 10 19:06:04 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [ 3466.716152] vethb1f5b1a: renamed from eth0
Aug 10 19:06:04 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [ 3466.743221] docker0: port 1(veth029f69d) entered disabled state
Aug 10 19:06:04 travis-job-dfa19e41-4c53-422a-a67b-1ef2f6685502 kernel: [ 3466.775527] docker0: port 13660756 .
2296984 ./obj/build
1693296 ./obj/build/x86_64-unknown-linux-gnu
792984 ./src
570240 ./.git
---
153744 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
149120 ./src/llvm-emscripten/test
147700 ./obj/build/bootstrap/debug/incremental
133268 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
133264 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f3qbi5e5ai-1uqnmt2-10j4kdvcqyom
128740 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
124756 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
124752 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
121992 ./obj/build/x86_64-unk
