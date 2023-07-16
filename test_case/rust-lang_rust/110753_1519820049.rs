plain
[RUSTC-TIMING] rustc_std_workspace_alloc test:false 0.030
   Compiling object v0.30.1
[RUSTC-TIMING] panic_abort test:false 0.098
[RUSTC-TIMING] panic_unwind test:false 0.109
error[E0425]: cannot find function `getauxval` in crate `libc`
   --> library/stdarch/crates/std_detect/src/detect/os/linux/auxvec.rs:148:40
    |
148 |             let hwcap = unsafe { libc::getauxval(AT_HWCAP as libc::c_ulong) as usize };

error[E0425]: cannot find function `getauxval` in crate `libc`
error[E0425]: cannot find function `getauxval` in crate `libc`
   --> library/stdarch/crates/std_detect/src/detect/os/linux/auxvec.rs:149:41
    |
149 |             let hwcap2 = unsafe { libc::getauxval(AT_HWCAP2 as libc::c_ulong) as usize };

For more information about this error, try `rustc --explain E0425`.
[RUSTC-TIMING] std_detect test:false 0.186
error: could not compile `std_detect` due to 2 previous errors
