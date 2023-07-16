plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0559]: variant `FileFlags::Elf` has no field named `os_abi`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:181:35
    |
181 |     file.flags = FileFlags::Elf { os_abi, e_flags };
    |                                   ^^^^^^ `FileFlags::Elf` does not have this field
    = note: available fields are: `e_flags`

    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
For more information about this error, try `rustc --explain E0559`.
