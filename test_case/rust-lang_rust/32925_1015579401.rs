console
$ rustc ba.rs -g
$ readelf -wi ba |grep identity
    <386>   DW_AT_linkage_name: (indirect string, offset: 0x42d): _ZN2ba8identity17h4032c34ace3f366cE
    <38a>   DW_AT_name        : (indirect string, offset: 0x451): identity<u64>
    <3bb>   DW_AT_linkage_name: (indirect string, offset: 0x45f): _ZN2ba8identity17h97d8ddefaee8e693E
    <3bf>   DW_AT_name        : (indirect string, offset: 0x483): identity<i32>
$ nm ba | grep identity
0000000000007c20 t _ZN2ba8identity17h4032c34ace3f366cE
0000000000007c30 t _ZN2ba8identity17h97d8ddefaee8e693E
