plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0026]: variant `rustc_middle::mir::TerminatorKind::Assert` does not have a field named `target`
   |
   |
93 |             TerminatorKind::Assert { ref mut target, .. }
   |                                              ^^^^^^ variant `rustc_middle::mir::TerminatorKind::Assert` does not have this field

error[E0026]: variant `rustc_middle::mir::TerminatorKind::Call` does not have a field named `destination`
   |
   |
94 |             | TerminatorKind::Call { destination: Some((_, ref mut target)), .. }
   |                                      ^^^^^^^^^^^ variant `rustc_middle::mir::TerminatorKind::Call` does not have this field

error[E0026]: variant `rustc_middle::mir::TerminatorKind::InlineAsm` does not have a field named `destination`
    |
    |
100 |             | TerminatorKind::InlineAsm { destination: Some(ref mut target), .. }
    |                                           ^^^^^^^^^^^ variant `rustc_middle::mir::TerminatorKind::InlineAsm` does not have this field

error[E0026]: variant `rustc_middle::mir::TerminatorKind::SwitchInt` does not have a field named `targets`
    |
    |
120 |             TerminatorKind::SwitchInt { ref mut targets, .. } => {
    |                                                 ^^^^^^^ variant `rustc_middle::mir::TerminatorKind::SwitchInt` does not have this field

error[E0559]: variant `rustc_middle::mir::TerminatorKind<'_>::Call` has no field named `func`
    |
146 |             TerminatorKind::Call {
146 |             TerminatorKind::Call {
    |             -------------------- `rustc_middle::mir::TerminatorKind<'_>` is a tuple variant, use the appropriate syntax: `rustc_middle::mir::TerminatorKind<'_>(/* fields */)`
147 |                 func: Operand::Copy(self.dummy_place.clone()),
    |                 ^^^^ field does not exist
   ::: /checkout/compiler/rustc_middle/src/mir/terminator.rs:228:5
    |
    |
228 |     Call(Box<CallTerminator<'tcx>>),
    |     ---- `rustc_middle::mir::TerminatorKind<'_>` defined here

error[E0559]: variant `rustc_middle::mir::TerminatorKind<'_>::Call` has no field named `args`
    |
146 |             TerminatorKind::Call {
146 |             TerminatorKind::Call {
    |             -------------------- `rustc_middle::mir::TerminatorKind<'_>` is a tuple variant, use the appropriate syntax: `rustc_middle::mir::TerminatorKind<'_>(/* fields */)`
147 |                 func: Operand::Copy(self.dummy_place.clone()),
148 |                 args: vec![],
    |                 ^^^^ field does not exist
   ::: /checkout/compiler/rustc_middle/src/mir/terminator.rs:228:5
    |
    |
228 |     Call(Box<CallTerminator<'tcx>>),
    |     ---- `rustc_middle::mir::TerminatorKind<'_>` defined here

error[E0559]: variant `rustc_middle::mir::TerminatorKind<'_>::Call` has no field named `destination`
    |
146 |             TerminatorKind::Call {
146 |             TerminatorKind::Call {
    |             -------------------- `rustc_middle::mir::TerminatorKind<'_>` is a tuple variant, use the appropriate syntax: `rustc_middle::mir::TerminatorKind<'_>(/* fields */)`
...
149 |                 destination: Some((self.dummy_place.clone(), TEMP_BLOCK)),
    |                 ^^^^^^^^^^^ field does not exist
   ::: /checkout/compiler/rustc_middle/src/mir/terminator.rs:228:5
    |
    |
228 |     Call(Box<CallTerminator<'tcx>>),
    |     ---- `rustc_middle::mir::TerminatorKind<'_>` defined here

error[E0559]: variant `rustc_middle::mir::TerminatorKind<'_>::Call` has no field named `cleanup`
    |
146 |             TerminatorKind::Call {
146 |             TerminatorKind::Call {
    |             -------------------- `rustc_middle::mir::TerminatorKind<'_>` is a tuple variant, use the appropriate syntax: `rustc_middle::mir::TerminatorKind<'_>(/* fields */)`
150 |                 cleanup: None,
150 |                 cleanup: None,
    |                 ^^^^^^^ field does not exist
   ::: /checkout/compiler/rustc_middle/src/mir/terminator.rs:228:5
    |
    |
228 |     Call(Box<CallTerminator<'tcx>>),
    |     ---- `rustc_middle::mir::TerminatorKind<'_>` defined here

error[E0559]: variant `rustc_middle::mir::TerminatorKind<'_>::Call` has no field named `from_hir_call`
    |
146 |             TerminatorKind::Call {
146 |             TerminatorKind::Call {
    |             -------------------- `rustc_middle::mir::TerminatorKind<'_>` is a tuple variant, use the appropriate syntax: `rustc_middle::mir::TerminatorKind<'_>(/* fields */)`
151 |                 from_hir_call: false,
151 |                 from_hir_call: false,
    |                 ^^^^^^^^^^^^^ field does not exist
   ::: /checkout/compiler/rustc_middle/src/mir/terminator.rs:228:5
    |
    |
228 |     Call(Box<CallTerminator<'tcx>>),
    |     ---- `rustc_middle::mir::TerminatorKind<'_>` defined here

error[E0559]: variant `rustc_middle::mir::TerminatorKind<'_>::Call` has no field named `fn_span`
    |
146 |             TerminatorKind::Call {
146 |             TerminatorKind::Call {
    |             -------------------- `rustc_middle::mir::TerminatorKind<'_>` is a tuple variant, use the appropriate syntax: `rustc_middle::mir::TerminatorKind<'_>(/* fields */)`
...
152 |                 fn_span: DUMMY_SP,
    |                 ^^^^^^^ field does not exist
   ::: /checkout/compiler/rustc_middle/src/mir/terminator.rs:228:5
    |
    |
228 |     Call(Box<CallTerminator<'tcx>>),
    |     ---- `rustc_middle::mir::TerminatorKind<'_>` defined here

error[E0559]: variant `rustc_middle::mir::TerminatorKind<'_>::SwitchInt` has no field named `discr`
    |
162 |         let switchint_kind = TerminatorKind::SwitchInt {
162 |         let switchint_kind = TerminatorKind::SwitchInt {
    |                              ------------------------- `rustc_middle::mir::TerminatorKind<'_>` is a tuple variant, use the appropriate syntax: `rustc_middle::mir::TerminatorKind<'_>(/* fields */)`
163 |             discr: Operand::Move(Place::from(self.new_temp())),
    |             ^^^^^ field does not exist
   ::: /checkout/compiler/rustc_middle/src/mir/terminator.rs:171:5
    |
    |
171 |     SwitchInt(Box<SwitchIntTerminator<'tcx>>),
    |     --------- `rustc_middle::mir::TerminatorKind<'_>` defined here

error[E0559]: variant `rustc_middle::mir::TerminatorKind<'_>::SwitchInt` has no field named `switch_ty`
    |
162 |         let switchint_kind = TerminatorKind::SwitchInt {
162 |         let switchint_kind = TerminatorKind::SwitchInt {
    |                              ------------------------- `rustc_middle::mir::TerminatorKind<'_>` is a tuple variant, use the appropriate syntax: `rustc_middle::mir::TerminatorKind<'_>(/* fields */)`
163 |             discr: Operand::Move(Place::from(self.new_temp())),
164 |             switch_ty: dummy_ty(),
    |             ^^^^^^^^^ field does not exist
   ::: /checkout/compiler/rustc_middle/src/mir/terminator.rs:171:5
    |
    |
171 |     SwitchInt(Box<SwitchIntTerminator<'tcx>>),
    |     --------- `rustc_middle::mir::TerminatorKind<'_>` defined here

error[E0559]: variant `rustc_middle::mir::TerminatorKind<'_>::SwitchInt` has no field named `targets`
    |
162 |         let switchint_kind = TerminatorKind::SwitchInt {
162 |         let switchint_kind = TerminatorKind::SwitchInt {
    |                              ------------------------- `rustc_middle::mir::TerminatorKind<'_>` is a tuple variant, use the appropriate syntax: `rustc_middle::mir::TerminatorKind<'_>(/* fields */)`
...
165 |             targets: SwitchTargets::static_if(0, TEMP_BLOCK, TEMP_BLOCK),
    |             ^^^^^^^ field does not exist
   ::: /checkout/compiler/rustc_middle/src/mir/terminator.rs:171:5
    |
    |
171 |     SwitchInt(Box<SwitchIntTerminator<'tcx>>),
    |     --------- `rustc_middle::mir::TerminatorKind<'_>` defined here

error[E0026]: variant `rustc_middle::mir::TerminatorKind::Assert` does not have a field named `target`
    |
191 |                     TerminatorKind::Assert { target, .. }
191 |                     TerminatorKind::Assert { target, .. }
    |                                              ^^^^^^ variant `rustc_middle::mir::TerminatorKind::Assert` does not have this field

error[E0026]: variant `rustc_middle::mir::TerminatorKind::Call` does not have a field named `destination`
    |
    |
192 |                     | TerminatorKind::Call { destination: Some((_, target)), .. }
    |                                              ^^^^^^^^^^^ variant `rustc_middle::mir::TerminatorKind::Call` does not have this field

error[E0026]: variant `rustc_middle::mir::TerminatorKind::InlineAsm` does not have a field named `destination`
    |
    |
198 |                     | TerminatorKind::InlineAsm { destination: Some(target), .. }
    |                                                   ^^^^^^^^^^^ variant `rustc_middle::mir::TerminatorKind::InlineAsm` does not have this field

error[E0026]: variant `rustc_middle::mir::TerminatorKind::SwitchInt` does not have a field named `targets`
    |
202 |                     TerminatorKind::SwitchInt { targets, .. } => {
202 |                     TerminatorKind::SwitchInt { targets, .. } => {
    |                                                 ^^^^^^^ variant `rustc_middle::mir::TerminatorKind::SwitchInt` does not have this field
error: aborting due to 17 previous errors

Some errors have detailed explanations: E0026, E0559.
For more information about an error, try `rustc --explain E0026`.
For more information about an error, try `rustc --explain E0026`.
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_save_analysis" "-p" "rustc_metadata" "-p" "rustc_attr" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_feature" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_hir" "-p" "rustc_error_codes" "-p" "rustc_hir_pretty" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_errors" "-p" "rustc_ast_pretty" "-p" "rustc_ast" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_interface" "-p" "rustc_mir_build" "-p" "rustc_builtin_macros" "-p" "rustc_typeck" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_incremental" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_symbol_mangling" "-p" "rustc_ty_utils" "-p" "rustc_plugin_impl" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:48
