plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0425]: cannot find value `ELFOSABI_CLOUDABI` in module `elf`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:177:14
    |
177 |         elf::ELFOSABI_CLOUDABI
    |              ^^^^^^^^^^^^^^^^^ not found in `elf`

error[E0559]: variant `FileFlags::Elf` has no field named `os_abi`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:187:35
    |
187 |     file.flags = FileFlags::Elf { os_abi, e_flags };
    |                                   ^^^^^^ `FileFlags::Elf` does not have this field
    = note: available fields are: `e_flags`

    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
Some errors have detailed explanations: E0425, E0559.
