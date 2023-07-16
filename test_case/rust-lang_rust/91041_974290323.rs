plain
[RUSTC-TIMING] build_script_build test:false 0.632
error: unused attribute
   --> library/core/src/../../stdarch/crates/core_arch/src/mips/msa.rs:501:5
    |
501 |     #[link_name = "llvm.mips.fexupl.w"]
    |
    |
    = note: `-D unused-attributes` implied by `-D warnings`
note: attribute also specified here
   --> library/core/src/../../stdarch/crates/core_arch/src/mips/msa.rs:504:5
    |
504 |     #[link_name = "llvm.mips.fexupl.d"]
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

[RUSTC-TIMING] core test:false 14.048
error: could not compile `core` due to previous error
