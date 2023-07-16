plain
[00:44:39] ....................................................................................................
[00:44:42] ....................................................................................................
[00:44:45] ....................................................................................................
[00:44:48] ....................................................................................................
[00:44:51] .......i...........F................................................................................
[00:44:59] .....i..............................................................................................
[00:45:02] .........ii.iii.....................................................................................
[00:45:04] ....................................................................................................
[00:45:06] ....................................................................................................
[00:45:06] ....................................................................................................
[00:45:09] ....................................................................................................
[00:45:11] ....................................................................................................
[00:45:13] ..........................................................................................i.........
[00:45:16] .........................................................i..........................................
[00:45:20] ....................................................................................................
[00:45:23] .............F......................................................................................
[00:45:28] ....................................................................................................
[00:45:31] ....................................................................................................
[00:45:34] ....................................................................................................
[00:45:37] ....................................................................................................
[00:45:37] ....................................................................................................
[00:45:40] .....................................................................F..............................
[00:45:46] ....................................................................................................
[00:45:49] ....................................................................................................
[00:45:52] ..........................i.........................................................................
[00:45:52] ..........................i.........................................................................
[00:45:55] ..............F...............................................................i.i..ii...............
[00:45:58] ....................................................................................................
[00:46:01] .................................................................F....i.............................
[00:46:07] ....................................................................................................
[00:46:10] ....................................................................................................
[00:46:13] ....................................................................................................
[00:46:16] .......i............................................................................................
---
[00:46:35] 
[00:46:35] ---- [ui] ui/coercion/coerce-unsafe-to-closure.rs stdout ----
[00:46:35] diff of stderr:
[00:46:35] 
[00:46:35] - error[E0277]: the trait bound `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}: std::ops::FnOnce<(&str,)>` is not satisfied
[00:46:35] + error[E0277]: the trait bound `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}: std::ops::FnOnce<(&str,)>` is not satisfied
[00:46:35] 2   --> $DIR/coerce-unsafe-to-closure.rs:12:40
[00:46:35] 3    |
[00:46:35] 4 LL |     let x: Option<&[u8]> = Some("foo").map(std::mem::transmute);
[00:46:35] 
[00:46:35] -    |                                        ^^^ the trait `std::ops::FnOnce<(&str,)>` is not implemented for `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
[00:46:35] +    |                                        ^^^ the trait `std::ops::FnOnce<(&str,)>` is not implemented for `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
[00:46:35] 7 error: aborting due to previous error
[00:46:35] 7 error: aborting due to previous error
[00:46:35] 8rectory \"/checkout/src/test/ui/invalid-module-declaration/auxiliary/foo\"\n\n"}
[00:46:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:35] {"message":"For more information about this error, try `rustc --explain E0583`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0583`.\n"}
[00:46:35] ------------------------------------------
[00:46:35] 
[00:46:35] thread '[ui] ui/invalid-module-declaration/invalid-module-declaration.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3161:9
[00:46:35] 
[00:46:35] 
[00:46:35] ---- [ui] ui/issues/issue-46209-private-enum-variant-reexport.rs stdout ----
[00:46:35] diff of stderr:
[00:46:35] 
[00:46:35] 16 LL |     crate enum Crewman {
[00:46:35] 17    |     ------------------ help: consider making the enum public: `pub enum Crewman`
[00:46:35] 18 
[00:46:35] - error: enum is private and its variants cannot be re-exported
[00:46:35] -    |
[00:46:35] -    |
[00:46:35] - LL |     pub use self::Professor::*;
[00:46:35] - ...
[00:46:35] - ...
[00:46:35] - LL |     enum Professor {
[00:46:35] -    |     -------------- help: consider making the enum public: `pub enum Professor`
[00:46:35] - 
[00:46:35] 28 error: variant `JuniorGrade` is private and cannot be re-exported
[00:46:35] 30    |
[00:46:35] 
[00:46:35] 
[00:4/test/ui/issues/issue-46209-private-enum-variant-reexport/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46209-private-enum-variant-reexport/auxiliary" "-A" "unused"
[00:46:35] ------------------------------------------
[00:46:35] 
[00:46:35] ------------------------------------------
[00:46:35] stderr:
[00:46:35] stderr:
[00:46:35] ------------------------------------------
[00:46:35] {"message":"enum is private and its variants cannot be re-exported","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-46209-private-enum-variant-reexport.rs","byte_start":824,"byte_end":845,"line_start":19,"line_end":19,"column_start":13,"column_end":34,"is_primary":true,"text":[{"text":"    pub use self::PettyOfficer::*;","highlight_start":13,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider making the enum public","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-46209-private-enum-variant-reexport.rs","byte_start":1182,"byte_end":1212,"line_start":36,"line_end":36,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    pub(in rank) enum PettyOfficer {","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":"pub enum PettyOfficer","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: enum is private and its variants cannot be re-exporrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "aFdEfSeVEEE"
[00:46:35] 4    = note: ld: unrecognized option '-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib'
[00:46:35] 5            ld: use the --help option for usage information
[00:46:35] 
[00:46:35] 
[00:46:35] The actual stderr differed from the expected stderr.
[00:46:35] The actual stderr differed from the expected stderr.
[00:46:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/nolink-with-link-args.stderr
[00:46:35] To update references, rerun the tests and pass the `--bless` flag
[00:46:35] To only update this specific test, also pass `--test-args nolink-with-link-args.rs`
[00:46:35] error: 1 errors occurred comparing output.
[00:46:35] status: exit code: 1
[00:46:35] status: exit code: 1
[00:46:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nolink-with-link-args.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "linker-flavor=ld" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/auxiliary" "-A" "unused"
[00:46:35] ------------------------------------------
[00:46:35] 
[00:46:35] ------------------------------------------
[00:46:35] stderr:
[00:46:35] stderr:
[00:46:35] ------------------------------------------
[00:46:35] {"message":"linking with `ld` failed: exit code: 1","code":null,"level":"error","spans":[],"children":[{"message":"\"ld\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a.nolink_with_link_args0-317d481089b8c8fe83113de504472633.rs.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a.nolink_with_link_args1-317d481089b8c8fe83113de504472633.rs.rcgu.o\" \"-o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a\" \"--gc-sections\" \"-pie\" \"-zrelro\" \"-znow\" \"-O1\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/auxiliary\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"--start-group\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-lstd-2339b911e3c09de8\" \"--end-group\" \"-Bstatic\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2866e3dedc0a56e0.rlib\" \"-Bdynamic\" \"-ldl\" \"-lrt\" \"-lpthread\" \"-lpthread\" \"-lgcc_s\" \"-lc\" \"-lm\" \"-lrt\" \"-lpthread\" \"-lutil\" \"-lutil\" \"-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,--enable-new-dtags\" \"aFdEfSeVEEE\"","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"ld: unrecognized option '-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib'\nld: use the --help option for usage information\n","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: linking with `ld` failed: exit code: 1\n   |\n   = note: \"ld\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a.nolink_with_link_args0-317d481089b8c8fe83113de504472633.rs.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a.nolink_with_link_args1-317d481089b8c8fe83113de504472633.rs.rcgu.o\" \"-o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a\" \"--gc-sections\" \"-pie\" \"-zrelro\" \"-znow\" \"-O1\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/auxiliary\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"--start-group\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-lstd-2339b911e3c09de8\" \"--end-group\" \"-Bstatic\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2866e3dedc0a56e0.rlib\" \"-Bdynamic\" \"-ldl\" \"-lrt\" \"-lpthread\" \"-lpthread\" \"-lgcc_s\" \"-lc\" \"-lm\" \"-lrt\" kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0] Initializing cgroup subsys io
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.422207] Initializing cgroup subsys memory
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.422934] Initializing cgroup subsys devices
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.423561] Initializing cgroup subsys freezer
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.424696] Initializing cgroup subsys net_cls
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.425365] Initializing cgroup subsys perf_event
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.426239] Initializing cgroup subsys net_prio
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.427450] Initializing cgroup subsys hugetlb
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.428780] Initializing cgroup subsys pids
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.429653] CPU: Physical Processor ID: 0
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.430477] CPU: Processor Core ID: 0
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.431540] mce: CPU supports 32 MCE banks
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.432832] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.434266] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  9
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.840052] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.861487] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.870219] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.877152] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.897116] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.901172] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.904950] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.908371] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.911448] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.933344] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.935024] vgaarb: loaded
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    0.935758] SCSI subsystem initialized
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 keremory corruption every 60 seconds
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.171990] audit: initializing netlink subsys (disabled)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.173055] audit: type=2000 audit(1533824413.611:1): initialized
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.176108] Initialise system trusted keyring
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.177585] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.178792] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.181279] zbud: loaded
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.182162] VFS: Disk quotas dquot_6.6.0
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.183751] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.185630] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.187375] fuse init (API version 7.23)
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.188592] Key type big_key registered
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    3.189452] Allocating IMA MOK and blacklist keyrings.
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7ab069522] raid6: sse2x4   xor()  8607 MB/s
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.070572] raid6: using algorithm sse2x4 gen() 12773 MB/s
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.071920] raid6: .... xor() 8607 MB/s, rmw enabled
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.072705] raid6: using ssse3x2 recovery algorithm
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.075023] xor: automatically using best checksumming function:
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.113456]    avx       : 22159.000 MB/sec
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.128568] Btrfs loaded
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.174255] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.175565] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.251854] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.259466] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.260282] EXT4-fs (sda1): recovery complete
Aug  9 14:20:22 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [    8.266210] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 14:20:rnel: [   12.025063] random: nonblocking pool is initialized
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-accounts: INFO Creating a new user account for me.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-accounts: INFO Created user account me.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-accounts: INFO Creating a new user account for bogdana.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-accounts: INFO Created user account bogdana.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-accounts: INFO Creating a new user account for aj.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-accounts: INFO Created user account aj.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-accounts: INFO Creating a new user account for asari.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-accounts: INFO Created user account asari.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 google-accounts: INFO Removing user packer.
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 14:20:23 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 pollinate: To re-seed this system again, use the -r|--res76dee3e-9739-485d-ac3c-a1cd7abcd951 ntpd[1830]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 ntpd[1830]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 ntpd[1830]: peers refreshed
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 ntpd[1830]: Listening on routing socket on fd #21 for interface updates
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 ntpd[1830]: Listening on routing socket on fd #21 for interface updates
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [   42.771072] init: plymouth-upstart-bridge main process ended, respawning
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 startup-script: INFO Found startup-script in metadata.
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 startup-script: INFO startup-script: job 1 at Thu Aug  9 17:30:00 2018
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 startup-script: INFO startup-script: Return code 0.
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 startup-script: INFO startup-script: Return code 0.
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 startup-script: INFO Finished running startup scripts.
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 ec2: 
Aug  9 14:20:53 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 ec2: #####################################ed forwarding state
Aug  9 14:22:30 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [  139.375802] docker0: port 1(veth464a662) entered disabled state
Aug  9 14:22:30 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [  139.468480] cgroup: docker-runc (4817) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 14:22:30 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [  139.468483] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 14:22:30 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [  139.544007] eth0: renamed from veth11ae8e6
Aug  9 14:22:30 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [  139.585806] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 14:22:30 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [  139.587045] docker0: port 1(veth464a662) entered forwarding state
Aug  9 14:22:30 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [  139.587067] docker0: port 1(veth464a662) entered forwarding state
Aug  9 14:22:30 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 kernel: [  139.587099] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 14:22:33 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 ntpd[1830]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  9 14:22:33 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 ntpd[1830]: Listen normally on 6 docker0 fe80::42:60ff:fee4:8e51 UDP 123
Aug  9 14:22:33 travis-job-b76dee3e-9739-485d-ac3c-a1cd7abcd951 ntpd[1830]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
