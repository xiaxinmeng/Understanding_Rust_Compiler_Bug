plain
[00:48:32] ....................................................................................................
[00:48:35] ....................................................................................................
[00:48:38] ....................................................................................................
[00:48:41] ....................................................................................................
[00:48:45] .........................i..............................FF...............................F..........
[00:48:48] .....FFF............................i...............................................................
[00:48:55] ....................................................................................................
no-bound/auxiliary" "-A" "unused"
[00:49:01] stdout:
[00:49:01] ------------------------------------------
[00:49:01] ------------------------------------------
[00:49:01] 
[00:49:01] ------------------------------------------
[00:49:01] stderr:
[00:49:01] ------------------------------------------
[00:49:01] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs","byte_start":1609,"byte_end":1632,"line_start":49,"line_end":49,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:49:9\n   |\nLL |         demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:01] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs","byte_start":1528,"byte_end":1688,"line_start":45,"line_end":50,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":47,"highlight_end":66},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"texeporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1696,"byte_end":1719,"line_start":51,"line_end":51,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:51:9\n   |\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:01] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1603,"byte_end":1784,"line_start":48,"line_end":53,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":47,"highlight_end":79},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"    });","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacemi-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:49:01] 
[00:49:01] ------------------------------------------
[00:49:01] 
[00:49:01] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:01] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:01] 
[00:49:01] ---- [ui] ui/nll/issue-50716.rs stdout ----
[00:49:01] 
[00:49:01] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:49:01] status: exit code: 101
[00:49:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-50716.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-50716/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-50716/auxiliary" "-A" "unused"
[00:49:01] ------------------------------------------
[00:49:01] 
[00:49:01] ------------------------------------------
[00:49:01] stderr:
[00:49:01] stderr:
[00:49:01] ------------------------------------------
[00:49:01] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:345:21
[00:49:01] 
[00:49:01] error: internal compiler error: unexpected panic
[00:49:01] 
[00:49:01] 
[00:49:01] note: the compiler unexpectedly panicked. this is a bug.
[00:49ast_reify.stderr
[00:49ast_reify.stderr
[00:49:01] To update references, rerun the tests and pass the `--bless` flag
[00:49:01] To only update this specific test, also pass `--test-args nll/mir_check_cast_reify.rs`
[00:49:01] error: 1 errors occurred comparing output.
[00:49:01] status: exit code: 1
[00:49:01] status: exit code: 1
[00:49:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/mir_check_cast_reify.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_reify/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_reify/auxiliary" "-A" "unused"
[00:49:01] ------------------------------------------
[00:49:01] 
[00:49:01] ------------------------------------------
[00:49:01] stderr:
[00:49:01] stderr:
[00:49:01] ------------------------------------------
[00:49:01] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/mir_check_cast_reify.rs","byte_start":1682,"byte_end":1685,"line_start":46,"line_end":46,"column_start":25,"column_end":28,"is_primary":true,"text":[{"text":"    let f: fn(_) -> _ = foo;","highlight_start":25,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/mir_c.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:01] ---- [ui] ui/nll/mir_check_cast_unsize.rs stdout ----
[00:49:01] diff of stderr:
[00:49:01] 
[00:49:01] 5    |     ^
[00:49:01] 5    |     ^
[00:49:01] 6 
[00:49:01] 7 error: unsatisfied lifetime constraints
[00:49:01] -   --> $DIR/mir_check_cast_unsize.rs:17:46
[00:49:01] +   --> $DIR/mir_check_cast_unsize.rs:19:5
[00:49:01] 9    |
[00:49:01] - LL |   fn bar<'a>(x: &'a u32) -> &'static dyn Debug {
[00:49:01] -    |  ________--____________________________________^
[00:49:01] -    | |        lifetime `'a` defined here
[00:49:01] -    | |        lifetime `'a` defined here
[00:49:01] - LL | |     //~^ ERROR unsatisfied lifetime constraints
[00:49:01] - LL | |     x
[00:49:01] - LL | |     //~^ WARNING not reporting region error due to nll
[00:49:01] - LL | | }
[00:49:01] -    | |_^ return requires that `'a` must outlive `'static`
[00:49:01] + LL | fn bar<'a>(x: &'a u32) -> &'static dyn Debug {
[00:49:01] +    |        -- lifetime `'a` defined here
[00:49:01] + LL |     //~^ ERROR unsatisfied lifetime constraints
[00:49:01] + LL |     x
[00:49:01] +    |     ^ cast requires that `'a` must outlive `'static`
[00:49:01] 20 error: aborting due to previous error
[00:49:01] 21 
[00:49:01] 
[00:49:01] 
[00:49:01] 
[00:49:01] The actual stderr differed from the expected stderr.
[00:49:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_unsize/mir_check_cast_unsize.stderr
[00:49:01] To update references, rerun the tests and pass the `--bless` flag
[00:49:01] To only update this specific test, also pass `--tes22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000]   Device   empty
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] Movable zone start for each node
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] Early memory node ranges
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000]   DMA03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] Policy zone: Normal
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd290 kernel: [    0.889847] .... node  #0, CPUs:      #1
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.891837] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.897800]  #2
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.899137] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.904868]  #3
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.906084] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.911951] x86: Booted up 1 node, 4 CPUs
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.914398] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.921484] devtmpfs: initialized
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.927045] evm: security.selinux
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.929271] evm: security.SMACK64
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.930689] evm: security.SMACK64EXEC
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.932641] evm: security.SMACK64TRANSMUTE
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    0.934484] evm: security.SMACK64MMAP
Aug 14 03:22:34 travis-job-505d1e04-a02f-406-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.007866] ACPI: Added _OSI(Module Device)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.009720] ACPI: Added _OSI(Processor Device)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.012016] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.014331] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.019244] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.045321] ACPI: Interpreter enabled
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.048336] ACPI: (supports S0 S3 S4 S5)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.050228] ACPI: Using IOAPIC for interrupt routing
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.053228] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.086708] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.090264] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.094076] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97-6b4ccb169f00 kernel: [    1.179390] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.189613] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.197828] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.218762] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.232229] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.240001] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.259804] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.264801] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.270187] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.276486] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.280856] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    1.303237] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb1605d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.524882] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.527981] ACPI: Sleep Button [SLPF]
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.530160] GHES: HEST is not enabled!
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.534539] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.537415] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.545866] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.548750] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.560564] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.585427] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.611063] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.637715] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.662817] 00:06: ttyS3 at I 3.788221] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.792595] NET: Registered protocol family 10
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.795013] NET: Registered protocol family 17
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.797247] Key type dns_resolver registered
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.799992] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.803803] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.806399] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.809767] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.812670] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.817788] registered taskstats version 1
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.820050] Loading compiled-in X.509 certificates
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    3.822611] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 9f00 kernel: [    8.747988] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    9.011514] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    9.159939] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    9.216011] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [    9.473662] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [   10.141262] random: cloud-init: uninitialized urandom read (32 bytes read, 42 bits of entropy available)
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [   10.294119] systemd-udevd[702]: starting version 204
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [   10.435872] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [   10.503945] intel_rapl: no valid rapl domains found in package 0
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [   10.562382] ppdev: user-space parallel port driver
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 kernel: [   10.709120] random: mktemp: uninitialized urandoing set_multiqueue.
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO Set channels for eth0 to 4.
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 03:22:34 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 instance-setup: INFO Setting /proc00 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 google-accounts: INFO Creating a new user account for me.
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 google-accounts: INFO Created user account me.
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 google-accounts: INFO Creating a new user account for henrik.
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 google-accounts: INFO Created user account henrik.
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 google-accounts: INFO Creating a new user account for emma.
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 cron[1404]: (CRON) INFO (pidfile fd = 3)
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 cron[1446]: (CRON) STARTUP (fork ok)
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 cron[1446]: (CRON) INFO (Running @reboot jobs)
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 03:22:35 travis-job-505d1e04-a02f-4068-a97e-6b4ccb169f00 google-accounts: INFO Created user account emma.
