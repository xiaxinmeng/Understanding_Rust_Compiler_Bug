
$ readelf -V obj/build/x86_64-unknown-illumos/lld/bin/lld
...
Version needs section '.gnu.version_r' contains 4 entries:
 Addr: 0x0000000000411308  Offset: 0x011308  Link: 4 (.dynstr)
  000000: Version: 1  File: libdl.so.1  Cnt: 1
  0x0010:   Name: SUNW_0.8  Flags: none  Version: 14
  0x0020: Version: 1  File: librt.so.1  Cnt: 3
  0x0030:   Name: SUNW_1.1  Flags: none  Version: 16
  0x0040:   Name: SUNW_1.2  Flags: none  Version: 13
  0x0050:   Name: SUNW_0.7  Flags: none  Version: 11
  0x0060: Version: 1  File: libm.so.2  Cnt: 2
  0x0070:   Name: SUNW_1.1  Flags: none  Version: 7
  0x0080:   Name: SUNW_1.2  Flags: none  Version: 3
  0x0090: Version: 1  File: libc.so.1  Cnt: 10
  0x00a0:   Name: SUNW_1.23  Flags: none  Version: 17
  0x00b0:   Name: SYSVABI_1.3  Flags: none  Version: 15
  0x00c0:   Name: SUNWprivate_1.1  Flags: none  Version: 12
  0x00d0:   Name: ILLUMOS_0.12  Flags: none  Version: 10
  0x00e0:   Name: SUNW_0.9  Flags: none  Version: 9
  0x00f0:   Name: SUNW_1.22  Flags: none  Version: 8
  0x0100:   Name: SUNW_1.1  Flags: none  Version: 6
  0x0110:   Name: ILLUMOS_0.8  Flags: none  Version: 5
  0x0120:   Name: SUNW_1.18  Flags: none  Version: 4
  0x0130:   Name: SUNW_0.7  Flags: none  Version: 2
