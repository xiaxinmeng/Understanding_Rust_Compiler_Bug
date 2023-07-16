\n\nPlease be sure that a file corresponding to the module exists. If you\nwant to use a module named `file_that_doesnt_exist`, you need to have a file\nnamed `file_that_doesnt_exist.rs` or `file_that_doesnt_exist/mod.rs` in the\nsame directory.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/invalid-module-declaration/auxiliary/foo/bar.rs","byte_start":475,"byte_end":478,"line_start":11,"line_end":11,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"pub mod baz;","highlight_start":13,"column_end":29,"is_primary":true,"text":[{"text":"    pub use self::Crewman::*;","highlight_start":13,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider making the enum public","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-46209-private-enum-variant-reexport.rs","byte_start":1302,"byte_end":1320,"line_start":43,"line_end":43,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    crate enum Crewman {","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":"pub enum Crewman","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: enum is private and its variants cannot be re-exported\n  --> /checkout/src/test/ui/issues/issue-46209-private-enum-variant-reexport.rs:21:13\n   |\nLL |     pub use self::Crewman::*;\n   |             ^^^^^^^^^^^^^^^^\n...\nLL |     crate enum Crewman {\n   |     ------------------ help: consider making the enum public: `pub enum Crewman`\n\n"}
[00:49:36] {"message":"variant `JuniorGrade` is private and cannot be re-exported","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-46209-private-enum-variant-reexport.rs","byte_start":651,"byte_end":662,"line_start":16,"line_end":16,"column_start":32,"column_end":43,"is_primary":true,"text":[{"text":"    pub use self::Lieutenant::{JuniorGrade, Full};","highlight_start":32,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"chjects/rust/workdirs/rust1/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-ce73107aecc35a63.rlib" "-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/home/david/projects/rust/workdirs/rust1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "aFdEfSeVEEE"
[00:49:36] +    = note: "ld" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a.nolink_with_link_args0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a.nolink_with_link_args1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/a" "--gc-sections" "-pie" "-zrelro" "-znow" "-O1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nolink-with-link-args/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-2339b911e3c09de8" "--end-group" "-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2866e3dedc0a56e0.rlib" "-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lpthread" "-lgcc_s"ot execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:36] 
[00:49:36] 
[00:49:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:36] Build completed unsuccessfully in 0:03:05
[00:49:36] Build completed unsuccessfully in 0:03:05
[00:49:36] Makefile:58: recipe for target 'check' failed
[00:49:36] make: *** [check] Error 1
f511d08abc3 kernel: [    0.380259] Freeing SMP alternatives memory: 32K
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.388535] ftrace: allocating 32185 entries in 126 pages
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.434971] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.436283] smpboot: Max logical packages: 2
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.437488] x2apic enabled
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.439522] Switched APIC routing to physical x2apic.
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.442885] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.550642] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.552223] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.554455] x86: Booting SMP configuration:
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.555063] .... node  #0, CPUs:      #1
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.555853] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [  1d08abc3 kernel: [    0.626343] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.627192] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.630968] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.654100] ACPI: Interpreter enabled
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.654875] ACPI: (supports S0 S3 S4 S5)
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.655708] ACPI: Using IOAPIC for interrupt routing
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.656440] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.686756] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.687747] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.688785] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.689865] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.692306] PCI host bridge to bus 0000:00.
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.808169] ACPI: bus type USB registered
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.808874] usbcore: registered new interface driver usbfs
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.809843] usbcore: registered new interface driver hub
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.810889] usbcore: registered new device driver usb
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.812022] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.813301] dmi: Firmware registration failed.
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.814147] PCI: Using ACPI for IRQ routing
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.814827] PCI: pci_cache_line_size set to 64 bytes
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.814921] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.814922] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.815064] NetLabel: Initializing
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.815868] NetLabel:  domain hash size = 128
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    0.816479] NetLabel8:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.827986] audit: initializing netlink subsys (disabled)
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.829206] audit: type=2000 audit(1533757123.513:1): initialized
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.830416] Initialise system trusted keyring
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.831509] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.832548] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.834716] zbud: loaded
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.835348] VFS: Disk quotas dquot_6.6.0
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.835973] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.837525] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.838790] fuse init (API version 7.23)
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.839559] Key type big_key registered
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.840374] Allocating IMA MOK and blacklist keyrings.
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [    2.842542] Key type asymmetr INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 19:38:50 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 19:38:51 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 19:38:51 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  8 19:38:51 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 19:38:51 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [ug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [   11.450078] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Created user account solarce.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Creating a new user account for asari.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [   11.506857] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [   11.511630] Bridge firewalling registered
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [   11.520468] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Created user account asari.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Creating a new user account for bogdana.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Created user account bogdana.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [   11.598795] Initializing XFRM netlink socket
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [   11.606841] Netfilter messages via NETLINK v0.30.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [   11.609093] ctnetlink v0.93: registering with nfnetlink.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Creating a new user account for konstantin.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Created user account konstantin.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Creating a new user account for carmen.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Created user account carmen.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Creating a new user account for maria.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Created user account maria.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 google-accounts: INFO Removing user packer.
Aug  8 19:38:52 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [   12.015201] floppy0: no floppy controllers found
Aug  8 19:39:15 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpdate[1856]: adjust time server 169.254.169.254 offset 0.005627 sec
Aug  8 19:39:22 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpd[1891]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 19:39:22 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpd[1892]: proto: precision = 0.106 usec
Aug  8 19:39:22 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpd[1892]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 19:39:22 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpd[1892]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 19:39:22 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpd[1892]: Listen and dr4653] cgroup: docker-runc (4879) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 19:40:59 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [  138.164656] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 19:40:59 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [  138.236189] eth0: renamed from vethe1f9535
Aug  8 19:40:59 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [  138.278903] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 19:40:59 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [  138.279997] docker0: port 1(vetha6f8f9e) entered forwarding state
Aug  8 19:40:59 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [  138.280015] docker0: port 1(vetha6f8f9e) entered forwarding state
Aug  8 19:40:59 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 kernel: [  138.280039] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 19:41:02 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpd[1892]: Listen normally on 5 docker0 fe80::42:94ff:fe60:3fb3 UDP 123
Aug  8 19:41:02 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpd[1892]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  8 19:41:02 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpd[1892]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 19:41:02 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpd[1892]: peers refreshed
Aug  8 19:41:02 travis-job-4a15f783-7a34-4e0b-8adf-3f511d08abc3 ntpd[1892]: new interface(s) found: waking up resolver
Aug  8 19:41:14 travis-job-4a15n/user
