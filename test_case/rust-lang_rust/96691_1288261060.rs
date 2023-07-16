
PS D:\dev\rust> cat .\tezt.rs
pub fn std(x: u32) -> bool {
    if let Some(s) = x.checked_add(77) {
        s <= 4 && x % 8 == 0
    } else {
        false
    }
}
PS D:\dev\rust> rustc +nightly -V
rustc 1.66.0-nightly (6e95b6da8 2022-10-22)
PS D:\dev\rust> rustc +nightly .\tezt.rs -C opt-level=3 --emit=obj -o tezt_nightly_1_66.obj --crate-type=lib
PS D:\dev\rust> dumpbin /DISASM .\tezt_nightly_1_66.obj
Microsoft (R) COFF/PE Dumper Version 14.30.30709.0
Copyright (C) Microsoft Corporation.  All rights reserved.


Dump of file .\tezt_nightly_1_66.obj

File Type: COFF OBJECT

_ZN4tezt3std17hde2cff71fc0afa81E:
  0000000000000000: 31 C0              xor         eax,eax
  0000000000000002: 89 CA              mov         edx,ecx
  0000000000000004: 83 C2 4D           add         edx,4Dh
  0000000000000007: 72 0B              jb          0000000000000014
  0000000000000009: 83 FA 05           cmp         edx,5
  000000000000000C: 73 06              jae         0000000000000014
  000000000000000E: F6 C1 07           test        cl,7
  0000000000000011: 0F 94 C0           sete        al
  0000000000000014: C3                 ret

  Summary

           0 .bss
           0 .data
          15 .text
