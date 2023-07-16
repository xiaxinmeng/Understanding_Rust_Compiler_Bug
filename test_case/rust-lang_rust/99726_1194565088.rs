plain
[RUSTC-TIMING] addr2line test:false 0.479
[RUSTC-TIMING] core test:false 25.175
[RUSTC-TIMING] object test:false 4.798
[RUSTC-TIMING] gimli test:false 4.844
error[E0412]: cannot find type `kinfo_file` in crate `libc`
     |
     |
1131 |             let info = Box::<libc::kinfo_file>::new_zeroed();


error[E0412]: cannot find type `kinfo_file` in crate `libc`
     |
     |
1133 |             info.kf_structsize = mem::size_of::<libc::kinfo_file>() as libc::c_int;

For more information about this error, try `rustc --explain E0412`.
[RUSTC-TIMING] std test:false 2.177
error: could not compile `std` due to 2 previous errors
