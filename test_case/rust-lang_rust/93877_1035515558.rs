plain
    Checking chalk-engine v0.76.0
error[E0061]: this function takes 3 arguments but 4 arguments were supplied
   --> compiler/rustc_target/src/asm/mod.rs:92:31
    |
53  |  / macro_rules! def_regs {
54  |  |     ($arch:ident $arch_reg:ident $arch_regclass:ident {
55  |  |         $(
56  |  |             $reg:ident: $class:ident $(, $extra_class:ident)* = [$reg_name:literal $(, $alias:literal)*] $(% $filter:ident)?,
...    |
92  |  |                             $($filter(_arch, _target_features, _target, _is_clobber)?;)?
    |  |                               |
    |  |                               expected 3 arguments
...    |
128 |  |     }
128 |  |     }
129 |  | }
    |  |_- in this expansion of `def_regs!`
   ::: compiler/rustc_target/src/asm/bpf.rs:58:1
    |
    |
58  | /  def_regs! {
59  | |      Bpf BpfInlineAsmReg BpfInlineAsmRegClass {
60  | |          r0: reg = ["r0"],
61  | |          r1: reg = ["r1"],
83  | |      }
84  | |  }
    | |__- in this macro invocation
    |
    |
note: function defined here
   --> compiler/rustc_target/src/asm/bpf.rs:46:4
    |
46  | fn only_alu32(
    |    ^^^^^^^^^^
47  |     _arch: InlineAsmArch,
    |     --------------------
48  |     target_features: &FxHashSet<Symbol>,
49  |     _target: &Target,
    |     ----------------

error[E0061]: this function takes 3 arguments but 4 arguments were supplied
error[E0061]: this function takes 3 arguments but 4 arguments were supplied
   --> compiler/rustc_target/src/asm/mod.rs:116:22
    |
53  |  / macro_rules! def_regs {
54  |  |     ($arch:ident $arch_reg:ident $arch_regclass:ident {
55  |  |         $(
56  |  |             $reg:ident: $class:ident $(, $extra_class:ident)* = [$reg_name:literal $(, $alias:literal)*] $(% $filter:ident)?,
...    |
116 |  |                 if $($filter(_arch, _target_features, _target, false).is_ok() &&)? true {
    |  |                      |
    |  |                      expected 3 arguments
...    |
128 |  |     }
128 |  |     }
129 |  | }
    |  |_- in this expansion of `def_regs!`
   ::: compiler/rustc_target/src/asm/bpf.rs:58:1
    |
    |
58  | /  def_regs! {
59  | |      Bpf BpfInlineAsmReg BpfInlineAsmRegClass {
60  | |          r0: reg = ["r0"],
61  | |          r1: reg = ["r1"],
83  | |      }
84  | |  }
    | |__- in this macro invocation
    |
    |
note: function defined here
   --> compiler/rustc_target/src/asm/bpf.rs:46:4
    |
46  | fn only_alu32(
    |    ^^^^^^^^^^
47  |     _arch: InlineAsmArch,
    |     --------------------
48  |     target_features: &FxHashSet<Symbol>,
49  |     _target: &Target,
    |     ----------------

    Checking gsgdt v0.1.2
