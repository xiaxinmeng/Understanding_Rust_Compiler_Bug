\n\nIn this case, `foo` is defined, but is not a struct, so Rust can't use it as\none.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/enums-are-namespaced-xc.rs","byte_start":721,"byte_end":722,"line_start":19,"line_end":19,"column_start":31,"column_end":32,"is_primary":true,"text":[{"text":"    let _ = namespaced_enums::C { a: 10 };","highlight_start":31,"highlight_end":32}],"label":"not found in `namespaced_enums`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0422]: cannot find struct, variant or union type `C` in module `namespaced_enums`\n  --> /checkout/src/test/ui/resolve/enums-are-namespaced-xc.rs:19:31\n   |\nLL |     let _ = namespaced_enums::C { a: 10 };\n   |                               ^ not found in `namespaced_enums`\n\n"}
[00:49:33] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:33] {"message":"Some errors occurred: E0422, E0425.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0422, E0425.\n"}
[00:49:33] {"message":"For more information about an error, try `rustc --explain E0422`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0422`.\n"}
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] thread '[ui] ui/resolve/enums-are-namespaced-xc.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:33] 
[00:49:33] 
[00:49:33] ---- [ui] ui/resolve/issue-21221-1.rs stdout ----
[00:49:33] diff of stderr:
[00:49:33] 
[00:49:33] 3    |
[00:49:33] 4 LL | impl Mul for Foo {
[00:49:33] - help: possible candidates are found in other modules, you can import them into scope
[00:49:33] -    |
[00:49:33] -    |
[00:49:33] - LL | use mul1::Mul;
[00:49:33] -    |
[00:49:33] - LL | use mul2::Mul;
[00:49:33] -    |
[00:49:33] - LL | use std::ops::Mul;
[00:49:33] 14 
[00:49:33] 14 
[00:49:33] 15 error[E0412]: cannot find type `Mul` in this scope
[00:49:33] 
[00:49:33] 17    |
[00:49:33] 17    |
[00:49:33] 18 LL | fn getMul() -> Mul {
[00:49:33] - help: possible candidates are found in other modules, you can import them into scope
[00:49:33] -    |
[00:49:33] -    |
[00:49:33] - LL | use mul1::Mul;
[00:49:33] -    |
[00:49:33] - LL | use mul2::Mul;
[00:49:33] -    |
[00:49:33] - LL | use mul3::Mul;
[00:49:33] -    |
[00:49:33] - LL | use mul4::Mul;
[00:49:33] - and 2 other candidates
[00:49:33] 31 
[00:49:33] 31 
[00:49:33] 32 error[E0405]: cannot find trait `ThisTraitReallyDoesntExistInAnyModuleReally` in this scope
[00:49:33] 
[00:49:33] 40    |
[00:49:33] 40    |
[00:49:33] 41 LL | impl Div for Foo {
[00:49:33] - help: possible candidate is found in another module, you can import it into scope
[00:49:33] -    |
[00:49:33] -    |
[00:49:33] - LL | use std::ops::Div;
[00:49:33] 47 
[00:49:33] 48 error: aborting due to 4 previous errors
[00:49:33] 49 
[00:49:33] 
[00:49:33] 
[00:49:33] 
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0c80634c
$ sudo tail -n 500 /var/log/syslog
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] kvm-clock: using sched offset of 1435725346 cycles
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000] Zone ranges:
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 10 23:55:06 travis-job-749139dc-5740-437vel iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.346043] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.349284] Freeing SMP alternatives memory: 32K
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.358923] ftrace: allocating 32185 entries in 126 pages
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.412362] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.413312] smpboot: Max logical packages: 2
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.414548] x2apic enabled
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.416557] Switched APIC routing to physical x2apic.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.420288] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.526809] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.529201] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.532471] x86: Booting SMP configuration:
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.533701] .... node  #0, CPUs:      #1
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.534899] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.539548]  #2
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.540190] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.545269]  #3
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.545846] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.550848] x86: Booted up 1 node, 4 CPUs
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.551923] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.555624] devtmpfs: initialized
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.560115] evm: security.selinux
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.561562] evm: security.SMACK64
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.562458] evm: security.SMACK64EXEC
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.563426] evm: security.SMACK64TRANSMUTE
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.564847] evm: security.SMACK64MMAP
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.566191] evm: security.ima
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.567185] evm: security.capability
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.568992] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.572132] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.574186] pinctrl core: initialized pinctrl subsystem
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.576049] RTC time: 23:54:54, date: 08/10/18
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.578328] NET: Registered protocol family 16
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.590889] cpuidle: using governor ladder
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.602870] cpuidle: using governor menu
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.604504] PCCT header not found.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.605887] ACPI: bus type PCI registered
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.607481] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.610173] PCI: Using configuration type 1 for base access
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab[    0.708614] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.715522] PCI host bridge to bus 0000:00
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.717831] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.721504] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.724772] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.728949] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.732427] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.735208] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.735662] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.760960] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.786921] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [939849] NetLabel: Initializing
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.941418] NetLabel:  domain hash size = 128
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.943911] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.946426] NetLabel:  unlabeled traffic allowed by default
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.949501] amd_nb: Cannot enumerate AMD northbridges
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.952629] clocksource: Switched to clocksource kvm-clock
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.961915] pnp: PnP ACPI init
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.963956] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.964022] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.964068] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.964117] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.964160] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.964203] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.964246] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.964415] pnp: PnP ACPI: found 7 devices
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.975781] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.980551] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.980553] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.980555] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.980556] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.980592] NET: Registered protocol family 2
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.983152] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.987306] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.991071] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.994703] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    0.997785] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    1.001760] NET: Registered protocol family 1
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    1.004398] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    1.007811] PCI: CLS 0 bytes, default 64
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    1.007861] Unpacking initramfs...
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.060843] Freeing initrd memory: 21432K
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.062911] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.066966] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.072255] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.076992] hw unit of domain pp0-core 2^-0 Joules
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.079662] hw unit of domain package 2^-0 Joules
Aug 10 23:55:06 travis-jo09] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.411821] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.417249] i8042: Warning: Keylock active
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.420611] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.423532] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.427438] mousedev: PS/2 mouse device common for all mice
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.431260] rtc_cmos 00:00: RTC can wake from S4
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.434957] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.438886] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.442981] i2c /dev entries driver
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.444872] device-mapper: uevent: version 1.0.3
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.448154] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.454216] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.459198] NET: Registered protocol family 10
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.462425] NET: Registered protocol family 17
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.464750] Key type dns_resolver registered
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.467748] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.472065] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.475890] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.479265] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.483349] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.488967] registered taskstats version 1
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.491288] Loading compiled-in X.509 certificates
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.495134] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.501785] zswap: loaded using pool lzo/zbud
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.507710] Key type trusted registered
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.515169] Key type encrypted registered
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.517970] ima: No TPM chip found, activating TPM-bypass!
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.521394] evm: HMAC attrs: 0x1
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.524048]   Magic number: 14:96:959
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.526884] memory memory10: hash matches
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.530499] rtc_cmos 00:00: setting system clock to 2018-08-10 23:54:57 UTC (1533945297)
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.535653] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.539441] EDD information not available.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.542612] PM: Hibernation image not present or could not be loaded.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.544187] Freeing unused kernel memory: 1496K
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [    3.547118] Write protecting the kernel read-only data: 14336k
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [nts daemon.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for me.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account me.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for henrik.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account henrik.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for emma.
Aug 10 23:55:06 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account emma.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for igor.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account igor.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account konstantinhaase.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for aj.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account aj.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for solarce.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account solarce.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for asari.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [   13.360913] floppy0: no floppy controllers found
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account asari.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for bogdana.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account bogdana.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [   13.470969] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [   13.472990] random: nonblocking pool is initialized
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [   13.476220] Bridge firewalling registered
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for konstantin.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [   13.493308] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [   13.542117] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account konstantin.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for carmen.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [   13.636112] Initializing XFRM netlink socket
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account carmen.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [   13.645183] Netfilter messages via NETLINK v0.30.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a kernel: [   13.648793] ctnetlink v0.93: registering with nfnetlink.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Creating a new user account for maria.
Aug 10 23:55:07 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a google-accounts: INFO Created user account maria.
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a startup-script: INFO Finished running startup scripts.
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a ec2: 
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a ec2: 
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a ec2: #############################################################
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a ec2: 1024 0c:f5:b4:b2:11:68:f9:81:f8:c7:4e:72:22:8d:a3:18  root@travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a (DSA)
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a ec2: 256 0a:11:50:8d:64:f3:10:b8:9d:e8:b5:6d:79:da:5b:79  root@travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a (ECDSA)
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a ec2: 256 ce:be:fc:6e:2b:48:90:0a:6c:5b:5f:8e:a7:83:10:d8  root@travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a (ED25519)
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a ec2: 2048 1c:f8:5a:cf:3e:dd:db:21:a5:a0:9e:70:21:c0:97:ff  root@travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a (RSA)
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 23:55:15 travis-job-749139dc-5740-437d-bab8-1274bfd9bc7a ec2: #############################################################
Aug 10 23:56:51 travis-job-749139ed state
travis_time:end:0c80634c:start=1533948385709969365,finish=1533948385724857679,duration=14888314
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
