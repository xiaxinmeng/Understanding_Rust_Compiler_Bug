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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_mir/src/transform/inline.rs at line 143:
         caller_body: &mut Body<'tcx>,
         callsite: &CallSite<'tcx>,
     ) -> Result<std::ops::Range<BasicBlock>, &'static str> {
-
         let callee_attrs = self.tcx.codegen_fn_attrs(callsite.callee.def_id());
         self.check_codegen_attributes(callsite, callee_attrs)?;
         self.check_mir_is_available(caller_body, &callsite.callee)?;
Diff in /checkout/compiler/rustc_mir/src/transform/inline.rs at line 174:
         caller_body: &Body<'tcx>,
         callee: &Instance<'tcx>,
     ) -> Result<(), &'static str> {
-
         if callee.def_id() == caller_body.source.def_id() {
             return Err("self-recursion");
Diff in /checkout/compiler/rustc_mir/src/transform/inline.rs at line 247:
             Ok(())
         }
     }
     }
-
 
     fn resolve_callsite(
         &self,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/inline.rs" "/checkout/compiler/rustc_mir/src/transform/mod.rs" "/checkout/compiler/rustc_mir/src/transform/remove_noop_landing_pads.rs" "/checkout/compiler/rustc_mir/src/transform/add_moves_for_packed_drops.rs" "/checkout/compiler/rustc_mir/src/transform/elaborate_drops.rs" "/checkout/compiler/rustc_mir/src/transform/early_otherwise_branch.rs" "/checkout/compiler/rustc_mir/src/transform/const_prop.rs" "/checkout/compiler/rustc_mir/src/transform/unreachable_prop.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
