
$ rustc --version
rustc 1.34.0 (91856ed52 2019-04-10)
$ rustc --crate-type staticlib -o librust134.a testcase.rs
$ readelf --debug-dump=info librust134.a |grep -C 5 panic_bounds_check
readelf: Warning: unable to apply unsupported reloc type 17 to section .debug_info
    <7db7>   DW_AT_call_line   : 49
 <2><7db8>: Abbrev Number: 0
 <1><7db9>: Abbrev Number: 34 (DW_TAG_subprogram)
    <7dba>   DW_AT_low_pc      : 0x0
    <7dc2>   DW_AT_high_pc     : 0x77
    <7dc6>   DW_AT_name        : (indirect string, offset: 0x6d21): panic_bounds_check
 <2><7dca>: Abbrev Number: 33 (DW_TAG_inlined_subroutine)
    <7dcb>   DW_AT_abstract_origin: <0x626a>
    <7dcf>   DW_AT_low_pc      : 0x36
    <7dd7>   DW_AT_high_pc     : 0x36
    <7ddb>   DW_AT_call_file   : 44
$
