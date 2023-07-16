
UNIT<.debug_info+0x0004bec9>: length = 0xbf, format = Dwarf32, version = 4, address_size = 8, abbrev_offset = 0x0
< 0><0x0000000b>  DW_TAG_compile_unit
                    DW_AT_producer              (indirect string, index 0x39dc): clang LLVM (rustc version 1.57.0-nightly (ac2d9fc50 2021-09-21))
                    DW_AT_language              DW_LANG_Rust
                    DW_AT_name                  (indirect string, index 0x39df): xtask/src/main.rs
                    DW_AT_GNU_dwo_name          (indirect string, index 0x39de): /home/philipc/code/rust/rust-analyzer/target/release/deps/xtask-be398824d5b61cec.write_json-94b9108f588c9a54.write_json.1ae45326-cgu.0.rcgu.o.rcgu.o
                    DW_AT_GNU_dwo_id            0xc6ed43dcb82fe226
< 1><0x0000001c>    DW_TAG_namespace
                      DW_AT_name                  (indirect string, index 0x0): core
< 2><0x0000001e>      DW_TAG_namespace
                        DW_AT_name                  (indirect string, index 0x43): ffi
< 3><0x00000020>        DW_TAG_enumeration_type
                          DW_AT_type                  0x00000031<.debug_info+0x0004befa>
                          DW_AT_enum_class            yes
                          DW_AT_name                  (indirect string, index 0x46): c_void
                          DW_AT_byte_size             0x00000001
                          DW_AT_alignment             0x00000001
< 4><0x00000028>          DW_TAG_enumerator
                            DW_AT_name                  (indirect string, index 0x44): __variant1
                            DW_AT_const_value           0x00000000
< 4><0x0000002b>          DW_TAG_enumerator
                            DW_AT_name                  (indirect string, index 0x45): __variant2
                            DW_AT_const_value           0x00000001
< 4><0x0000002e>          DW_TAG_null
< 3><0x0000002f>        DW_TAG_null
< 2><0x00000030>      DW_TAG_null
< 1><0x00000031>    DW_TAG_base_type
                      DW_AT_name                  (indirect string, index 0x4): u8
                      DW_AT_encoding              DW_ATE_unsigned
                      DW_AT_byte_size             0x00000001
< 1><0x00000035>    DW_TAG_namespace
                      DW_AT_name                  (indirect string, index 0x17): std
< 2><0x00000037>      DW_TAG_namespace
                        DW_AT_name                  (indirect string, index 0x18): io
< 3><0x00000039>        DW_TAG_namespace
                          DW_AT_name                  (indirect string, index 0x19): error
< 4><0x0000003b>          DW_TAG_enumeration_type
                            DW_AT_type                  0x00000031<.debug_info+0x0004befa>
                            DW_AT_enum_class            yes
                            DW_AT_name                  (indirect string, index 0x42): ErrorKind
                            DW_AT_byte_size             0x00000001
                            DW_AT_alignment             0x00000001
< 5><0x00000043>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x1a): NotFound
                              DW_AT_const_value           0x00000000
< 5><0x00000046>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x1b): PermissionDenied
                              DW_AT_const_value           0x00000001
< 5><0x00000049>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x1c): ConnectionRefused
                              DW_AT_const_value           0x00000002
< 5><0x0000004c>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x1d): ConnectionReset
                              DW_AT_const_value           0x00000003
< 5><0x0000004f>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x1e): HostUnreachable
                              DW_AT_const_value           0x00000004
< 5><0x00000052>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x1f): NetworkUnreachable
                              DW_AT_const_value           0x00000005
< 5><0x00000055>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x20): ConnectionAborted
                              DW_AT_const_value           0x00000006
< 5><0x00000058>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x21): NotConnected
                              DW_AT_const_value           0x00000007
< 5><0x0000005b>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x22): AddrInUse
                              DW_AT_const_value           0x00000008
< 5><0x0000005e>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x23): AddrNotAvailable
                              DW_AT_const_value           0x00000009
< 5><0x00000061>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x24): NetworkDown
                              DW_AT_const_value           0x0000000a
< 5><0x00000064>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x25): BrokenPipe
                              DW_AT_const_value           0x0000000b
< 5><0x00000067>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x26): AlreadyExists
                              DW_AT_const_value           0x0000000c
< 5><0x0000006a>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x27): WouldBlock
                              DW_AT_const_value           0x0000000d
< 5><0x0000006d>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x28): NotADirectory
                              DW_AT_const_value           0x0000000e
< 5><0x00000070>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x29): IsADirectory
                              DW_AT_const_value           0x0000000f
< 5><0x00000073>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x2a): DirectoryNotEmpty
                              DW_AT_const_value           0x00000010
< 5><0x00000076>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x2b): ReadOnlyFilesystem
                              DW_AT_const_value           0x00000011
< 5><0x00000079>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x2c): FilesystemLoop
                              DW_AT_const_value           0x00000012
< 5><0x0000007c>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x2d): StaleNetworkFileHandle
                              DW_AT_const_value           0x00000013
< 5><0x0000007f>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x2e): InvalidInput
                              DW_AT_const_value           0x00000014
< 5><0x00000082>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x2f): InvalidData
                              DW_AT_const_value           0x00000015
< 5><0x00000085>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x30): TimedOut
                              DW_AT_const_value           0x00000016
< 5><0x00000088>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x31): WriteZero
                              DW_AT_const_value           0x00000017
< 5><0x0000008b>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x32): StorageFull
                              DW_AT_const_value           0x00000018
< 5><0x0000008e>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x33): NotSeekable
                              DW_AT_const_value           0x00000019
< 5><0x00000091>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x34): FilesystemQuotaExceeded
                              DW_AT_const_value           0x0000001a
< 5><0x00000094>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x35): FileTooLarge
                              DW_AT_const_value           0x0000001b
< 5><0x00000097>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x36): ResourceBusy
                              DW_AT_const_value           0x0000001c
< 5><0x0000009a>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x37): ExecutableFileBusy
                              DW_AT_const_value           0x0000001d
< 5><0x0000009d>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x38): Deadlock
                              DW_AT_const_value           0x0000001e
< 5><0x000000a0>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x39): CrossesDevices
                              DW_AT_const_value           0x0000001f
< 5><0x000000a3>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x3a): TooManyLinks
                              DW_AT_const_value           0x00000020
< 5><0x000000a6>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x3b): FilenameTooLong
                              DW_AT_const_value           0x00000021
< 5><0x000000a9>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x3c): ArgumentListTooLong
                              DW_AT_const_value           0x00000022
< 5><0x000000ac>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x3d): Interrupted
                              DW_AT_const_value           0x00000023
< 5><0x000000af>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x3e): Unsupported
                              DW_AT_const_value           0x00000024
< 5><0x000000b2>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x3f): UnexpectedEof
                              DW_AT_const_value           0x00000025
< 5><0x000000b5>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x40): OutOfMemory
                              DW_AT_const_value           0x00000026
< 5><0x000000b8>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x14): Other
                              DW_AT_const_value           0x00000027
< 5><0x000000bb>            DW_TAG_enumerator
                              DW_AT_name                  (indirect string, index 0x41): Uncategorized
                              DW_AT_const_value           0x00000028
< 5><0x000000be>            DW_TAG_null
< 4><0x000000bf>          DW_TAG_null
< 3><0x000000c0>        DW_TAG_null
< 2><0x000000c1>      DW_TAG_null
< 1><0x000000c2>    DW_TAG_null
