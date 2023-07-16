
$ rustc --version
rustc 1.33.0 (2aa4c46cf 2019-02-28)
$ cat > testcase.rs <<EOF
pub fn frob(a: &[usize]) -> usize { a[0] }
EOF
$ rustc --crate-type=staticlib -o librust133.a testcase.rs
$ readelf --debug-dump=info librust133.a |grep -C 5 panic_bounds_check
readelf: Warning: unable to apply unsupported reloc type 17 to section .debug_info
 <4><1b7fc>: Abbrev Number: 0
 <3><1b7fd>: Abbrev Number: 44 (DW_TAG_subprogram)
    <1b7fe>   DW_AT_low_pc      : 0x0
    <1b806>   DW_AT_high_pc     : 0x83
    <1b80a>   DW_AT_frame_base  : 1 byte block: 57 	(DW_OP_reg7 (rsp))
    <1b80c>   DW_AT_linkage_name: (indirect string, offset: 0x17710): _ZN4core9panicking18panic_bounds_check17h34b80e64d41db052E
    <1b810>   DW_AT_name        : (indirect string, offset: 0x1774b): panic_bounds_check
    <1b814>   DW_AT_decl_file   : 41
    <1b815>   DW_AT_decl_line   : 55
    <1b816>   DW_AT_external    : 1
    <1b816>   DW_AT_noreturn    : 1
 <4><1b816>: Abbrev Number: 11 (DW_TAG_inlined_subroutine)
$
