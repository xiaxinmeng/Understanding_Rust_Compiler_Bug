plain
   Compiling rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0583]: file not found for module `bpf`
   --> compiler/rustc_target/src/asm/mod.rs:154:1
    |
154 | mod bpf;
    |
    |
    = help: to create the module `bpf`, create file "compiler/rustc_target/src/asm/bpf.rs"

error[E0432]: unresolved imports `bpf::BpfInlineAsmReg`, `bpf::BpfInlineAsmRegClass`
   --> compiler/rustc_target/src/asm/mod.rs:165:15
    |
165 | pub use bpf::{BpfInlineAsmReg, BpfInlineAsmRegClass};
    |               ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^ no `BpfInlineAsmRegClass` in `asm::bpf`
    |               |
    |               no `BpfInlineAsmReg` in `asm::bpf`

error[E0425]: cannot find function `regclass_map` in module `bpf`
   --> compiler/rustc_target/src/asm/mod.rs:677:32
    |
677 |             let mut map = bpf::regclass_map();
    |                                ^^^^^^^^^^^^ not found in `bpf`
help: consider importing one of these items
    |
1   | use crate::asm::aarch64::regclass_map;
    |
---
1   | use crate::asm::mips::regclass_map;
    |
      and 5 other candidates

error[E0425]: cannot find function `fill_reg_map` in module `bpf`
   --> compiler/rustc_target/src/asm/mod.rs:678:18
    |
678 |             bpf::fill_reg_map(arch, has_feature, target, &mut map);
    |                  ^^^^^^^^^^^^ not found in `bpf`
help: consider importing one of these items
    |
1   | use crate::asm::aarch64::fill_reg_map;
    |
    |
1   | use crate::asm::arm::fill_reg_map;
1   | use crate::asm::hexagon::fill_reg_map;
    |
1   | use crate::asm::mips::fill_reg_map;
    |
