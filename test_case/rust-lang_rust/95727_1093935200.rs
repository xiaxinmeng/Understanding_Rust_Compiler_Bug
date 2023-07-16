plain
[RUSTC-TIMING] core test:false 18.583
[RUSTC-TIMING] addr2line test:false 0.429
[RUSTC-TIMING] gimli test:false 4.638
[RUSTC-TIMING] object test:false 4.771
error: function is never used: `cvt_nz`
    |
    |
218 | pub fn cvt_nz(error: libc::c_int) -> crate::io::Result<()> {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] std test:false 3.963
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:10:30
