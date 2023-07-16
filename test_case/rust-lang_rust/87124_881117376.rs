shell
andy@DESKTOP-IUH6DM6:~/uefi-rs/uefi-test-runner$ ./build.py --target x86_64 --headless --ci run
       Fresh core v0.0.0 (/home/andy/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/src/rust/library/core)
       Fresh unicode-xid v0.2.2
       Fresh rustc-std-workspace-core v1.99.0 (/home/andy/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/src/rust/library/rustc-std-workspace-core)
       Fresh compiler_builtins v0.1.46
       Fresh alloc v0.0.0 (/home/andy/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/src/rust/library/alloc)
       Fresh proc-macro2 v1.0.27
       Fresh quote v1.0.9
       Fresh cfg-if v1.0.0
       Fresh bit_field v0.10.1
       Fresh bitflags v1.2.1
       Fresh qemu-exit v2.0.1
       Fresh rlibc v1.0.0
       Fresh syn v1.0.73
       Fresh ucs2 v0.3.2
       Fresh log v0.4.14
       Fresh uefi-macros v0.3.3 (/home/andy/uefi-rs/uefi-macros)
       Fresh uefi v0.11.0 (/home/andy/uefi-rs)
       Fresh uefi-services v0.8.0 (/home/andy/uefi-rs/uefi-services)
       Fresh uefi-test-runner v0.2.0 (/home/andy/uefi-rs/uefi-test-runner)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
BdsDxe: loading Boot0001 "UEFI QEMU HARDDISK QM00001 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x0,0xFFFF,0x0)
BdsDxe: starting Boot0001 "UEFI QEMU HARDDISK QM00001 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x0,0xFFFF,0x0)
INFO: UEFI 2.7
INFO: Testing boot services
INFO: Testing memory functions
INFO: Allocating some pages of memory
INFO: Allocating a vector through the `alloc` crate
INFO: Allocating a structure with alignment to 0x100
INFO: Testing the `memmove` / `set_mem` functions
INFO: Testing memory map functions
INFO: Testing timer...
INFO: Testing watchdog...
INFO: Testing various protocols
INFO: Testing console protocols
INFO: Running text output protocol test
INFO: UEFI standard output current mode: Some(OutputMode { index: 0, dims: (80, 25) })
INFO: # uefi-rs test runner
INFO: Cursor visibility control unavailable
INFO: - Text mode #0: 25 rows by 80 columns
INFO: - Text mode #1: 31 rows by 100 columns
INFO: Running serial protocol test
INFO: Running graphics output protocol test
SCREENSHOT: gop_test
INFO: Running pointer protocol test
INFO: Pointer state has not changed since the last query
INFO: Running UEFI debug connection protocol test
INFO: - Architecture: EBC
INFO: Testing Media Access protocols
INFO: Root directory entry: NamedFileProtocolInfo { header: FileInfoHeader { size: 88, file_size: 8192, physical_size: 8192, create_time: 2021-7-16 11:34:32.0 2047 (empty), last_access_time: 2021-7-16 0:0:0.0 2047 (empty), modification_time: 2021-7-16 11:34:32.0 2047 (empty), attribute: DIRECTORY }, name: ['E', 'F', 'I', '\u{0}'] }
INFO: MBR partition: MbrPartitionRecord { boot_indicator: 128, starting_chs: [1, 1, 0], os_type: MbrOsType(6), ending_chs: [15, 255, 255], starting_lba: 63, size_in_lba: 1032129 }
INFO: Testing Platform Initialization protocols
INFO: Running shim lock protocol test
INFO: Shim lock protocol is not supported
INFO: Testing runtime services
INFO: Testing set_variable
INFO: Testing get_variable_size
INFO: Testing get_variable
INFO: Testing complete, shutting down...
andy@DESKTOP-IUH6DM6:~/uefi-rs/uefi-test-runner$
