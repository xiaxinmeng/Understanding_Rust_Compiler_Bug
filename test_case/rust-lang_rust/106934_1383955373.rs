plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0423]: expected value, found module `ty`
   --> src/base.rs:788:63
    |
788 |                     let layout = fx.layout_of(fx.monomorphize(ty));
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::ty;
3   | use rustc_span::sym::ty;
    |

error[E0425]: cannot find value `bx` in this scope
    |
    |
793 |                         layout = layout.field(bx.cx(), field.index());
    |
help: a local variable with a similar name exists
    |
    |
793 |                         layout = layout.field(fx.cx(), field.index());
help: consider importing this unit variant
    |
    |
3   | use rustc_target::asm::X86InlineAsmReg::bx;

error[E0308]: mismatched types
   --> src/base.rs:798:43
    |
    |
798 |                     lval.write_cvalue(fx, offset);
    |                          ------------     ^^^^^^ expected struct `value_and_place::CValue`, found `u64`
    |                          arguments to this function are incorrect
    |
note: associated function defined here
   --> src/value_and_place.rs:476:19
   --> src/value_and_place.rs:476:19
    |
476 |     pub(crate) fn write_cvalue(self, fx: &mut FunctionCx<'_, '_, 'tcx>, from: CValue<'tcx>) {

Some errors have detailed explanations: E0308, E0423, E0425.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` due to 3 previous errors
