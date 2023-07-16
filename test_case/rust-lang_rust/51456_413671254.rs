plain
[00:50:26] ....................................................................................................
[00:50:29] ....................................................................................................
[00:50:31] ....................................................................................................
[00:50:35] ....................................................................................................
[00:50:38] .i.....................................................................F............................
[00:50:45] ....................................................................................................
[00:50:48] ..............................i.....................................................................
[00:50:52] ....................................................................................................
[00:50:55] ....................................................................................................
[00:50:55] ....................................................................................................
[00:50:57] ...........................................................................i........................
on_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use crate::std::collections::hash_set::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use crate::std::collections::vec_deque::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use crate::std::string::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use crate::std::vec::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::collections::binary_heap::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::collections::hash_map::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::collections::hash_set::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::collections::vec_deque::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::string::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs","byte_start":625,"byte_end":625,"line_start":16,"line_end":16,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod plumbing {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::vec::Drain;\n\n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0422]: cannot find struct, variant or union type `Drain` in this scope\n  --> /checkout/src/test/ui/rust-2018/issue-52202-use-suggestions.rs:21:14\n   |\nLL |     let _d = Drain {};\n   |              ^^^^^ not found in this scope\nhelp: possible candidates are found in other modules, you can import them into scope\n   |\nLL | use crate::plumbing::Drain;\n   |\nLL | use crate::std::collections::binary_heap::Drain;\n   |\nLL | use crate::std::collections::hash_map::Drain;\n   |\nLL | use crate::std::collections::hash_set::Drain;\n   |\nand 9 other candidates\n\n"}
[00:50:58] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:58] {"message":"For more information about this error, try `rustc --explain E0422`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0422`.\n"}
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] thread '[ui] ui/rust-2018/issue-52202-use-suggestions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:50:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:58] 
[00:50:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:50:58] 
[00:50:58] 
[00:50:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:58] 
[00:50:58] 
[00:50:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:58] Build completed unsuccessfully in 0:03:17
[00:50:58] Build completed unsuccessfully in 0:03:17
[00:50:58] make: *** [check] Error 1
[00:50:58] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0962f450
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:304de0fd
$ sudo tail -n 500 /var/log/syslog
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000]   6 disabled
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000]   7 disabled
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] Using GB pages for direct mapping
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] kvm-clock: using sched offset of 1547347238 cycles
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] Zone ranges:
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000]   Device   empty
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] Movable zone start for each node
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] Early memory node ranges
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000]   node   0: [memects=0, CPUs=4, Nodes=1
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] Hierarchical RCU implementation.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] console [ttyS0] enabled
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.373628] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.374871] pid_max: default: 32768 minimum: 301
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.375567] ACPI: Core revision 20150930
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.381902] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-1411894:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.482514] Switched APIC routing to physical x2apic.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.486050] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.592786] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.594591] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.597027] x86: Booting SMP configuration:
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.597714] .... node  #0, CPUs:      #1
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.598548] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.602022]  #2
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.602618] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.606209]  #3
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.606719] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.610107] x86: Booted up 1 node, 4 CPUs
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.610998] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.613226] devtmpfs: initialized
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.617551] evm: security.selinux
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.618059] evm: security.SMACK64
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.618631] evm: security.SMACK64EXEC
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.619255] evm: security.SMACK64TRANSMUTE
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.619851] evm: security.SMACK64MMAP
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.620384] evm: security.ima
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.620963] evm: security.capability
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.621827] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.623167] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.624252] pinctrl core: initialized pinctrl subsystem
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.625332] RTC time: 19:24:14, date: 08/16/18
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.626885] NET: Registered protocol family 16
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.636799] cpuidle: using governor ladder
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.648799] cpuidle: using governor menu
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.649693] PCCT header not found.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.650444] ACPI: bus type PCI registered
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.651002] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.652125] PCI: Using configuration type 1 for base access
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.665603] ACPI: Added _OSI(Module Device)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.666265] ACPI: Added _OSI(Processor Device)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.666939] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.667785] ACPI: Added _OSI(Processor Aggregator Device)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.671058] ACPI: Executed 2 blocks of module-level executable AML code
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.693830] ACPI: Interpreter enabled
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.694442] ACPI: (supports S0 S3 S4 S5)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.694985] ACPI: Using IOAPIC for interrupt routing
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.695784] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.724648] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.725587] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.726623] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.727563] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.729778] PCI host bridge to bus 0000:00
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.730501] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.731520] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.732440] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.733490] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.734645] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.735517] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.735924] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.750105] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.762539] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.763928] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.769593] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.773992] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.785769] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.790645] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    0.794503] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernelb287a kernel: [    2.837655] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.840019] zbud: loaded
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.840854] VFS: Disk quotas dquot_6.6.0
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.841696] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.843049] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.844666] fuse init (API version 7.23)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.845484] Key type big_key registered
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.846143] Allocating IMA MOK and blacklist keyrings.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.847938] Key type asymmetric registered
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.848623] Asymmetric key parser 'x509' registered
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.849325] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.850440] io scheduler noop registered
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    2.851147] io scheduler deadline registered (default)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4: [    3.007611] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.009152] ehci-pci: EHCI PCI platform driver
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.010462] ehci-platform: EHCI generic platform driver
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.012063] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.014250] ohci-pci: OHCI PCI platform driver
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.015540] ohci-platform: OHCI generic platform driver
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.017160] uhci_hcd: USB Universal Host Controller Interface driver
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.018383] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.020773] i8042: Warning: Keylock active
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.022401] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.023469] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.024730] mousedev: PS/2 mouse device common for all mice
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    3.026279] rtc_cmos 00:0.055907] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    6.146994] floppy0: no floppy controllers found
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.322795] raid6: sse2x1   gen()  8632 MB/s
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.390788] raid6: sse2x1   xor()  6498 MB/s
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.458804] raid6: sse2x2   gen() 10517 MB/s
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.526791] raid6: sse2x2   xor()  7138 MB/s
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.594797] raid6: sse2x4   gen() 12287 MB/s
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.662788] raid6: sse2x4   xor()  8518 MB/s
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.665523] raid6: using algorithm sse2x4 gen() 12287 MB/s
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.669455] raid6: .... xor() 8518 MB/s, rmw enabled
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.672179] raid6: using ssse3x2 recovery algorithm
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.676949] xor: automatically using best checksumming function:
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.718752]    avx       : 26964.000 MB/sec
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.736285] Btrfs loaded
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.803469] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.807854] EXT4-fs (sda1): write access will be enabled during recovery
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.908599] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.920071] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.922776] EXT4-fs (sda1): recovery complete
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    7.932138] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    8.184515] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    8.330343] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    8.388385] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    8.623026] random: cloud-init: uninitialized urandom read (32 bytes read, 33 bits of entropy available)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    9.273397] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    9.419771] systemd-udevd[702]: starting version 204
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    9.542695] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    9.610281] intel_rapl: no valid rapl domains found in package 0
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    9.663546] intel_rapl: no valid rapl domains found in package 0
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    9.669963] ppdev: user-space parallel port driver
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    9.773964] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    9.830138] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [    9.901037] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [   10.071026] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [   10.437748] random: mktemp: uninitialized urandom read (12 bytes read, 55 bits of entropy available)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [   10.518982] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [   10.605941] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [   10.660305] EXT4-fs (sda1): resized filesystem to 7864064
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [   11.017444] init: failsafe main process (1094) killed by TERM signal
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Running set_multiqueue.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Set channels for eth0 to 4.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 16 19:24:24 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 16 19:24:25 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 19:24:25 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a google-clock-skew: INFO Clock drift token has changed: 0.
Aug 16 19:24:25 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a google-accounts: INFO Starting Google Accounts daemon.
Aug 16 19:24:25 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 16 19:24:25 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a google-accounts: INFO Creating a new user account for me.
Aug 16 19:24:25 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a google-accounts: INFO Created user account me.
Aug 16 19:24:25 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a google-accounts: INFO Creating a new user account for bogdana.
Aug 16 19:24:25 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a google-accounts: INFO Created user account bogdana.
Aug 16 19:24:25 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287arnel: [  185.536539] eth0: renamed from veth4d383b5
Aug 16 19:27:19 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [  185.570038] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 16 19:27:19 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [  185.571340] docker0: port 1(veth9aeefdc) entered forwarding state
Aug 16 19:27:19 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [  185.571372] docker0: port 1(veth9aeefdc) entered forwarding state
Aug 16 19:27:19 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [  185.571401] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 16 19:27:22 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a ntpd[1772]: Listen normally on 5 docker0 fe80::42:17ff:fe2a:4e77 UDP 123
Aug 16 19:27:22 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a ntpd[1772]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 16 19:27:22 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a ntpd[1772]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 16 19:27:22 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a ntpd[1772]: peers refreshed
Aug 16 19:27:22 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a ntpd[1772]: new interface(s) found: waking up resolver
Aug 16 19:27:34 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [  200.595886] docker0: port 1(veth9aeefdc) entered forwarding state
Aug 16 20:16:28 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [ 3135.227353] veth4d383b5: renamed from eth0
Aug 16 20:16:28 travis-job-e0df03c1-8a18-4ad8-8571-141189eb287a kernel: [ 3135.263323] docker0: port 1(veth9aeefdc) entered disabled state
Aug 16 20:16:28 tra4154304 .
2318988 ./obj/build
1713212 ./obj/build/x86_64-unknown-linux-gnu
1058240 ./src
776528 ./.git
---
151200 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148684 ./obj/build/bootstrap/debug/incremental
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
134248 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f3wzjnuwa3-1pxb3nb-19p1no4jx31q5
1
